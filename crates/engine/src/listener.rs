//! HTTP pairing and WebSocket RPC share an engine listener.
use crate::pairing::PairingStore;
use base64::Engine as _;
use bytes::Bytes;
use http_body_util::{BodyExt, Full, Limited};
use hyper::{Request, Response, StatusCode, body::Incoming, service::service_fn};
use hyper_util::rt::{TokioIo, TokioTimer};
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::{handshake::derive_accept_key, protocol::Role};

type Reply = Response<Full<Bytes>>;

/// Serve a previously bound local socket, preserving credential-free native IPC.
pub async fn serve_listener(
    listener: TcpListener,
    service: Arc<dyn roboco_rpc::RpcService>,
    pairing: PairingStore,
) {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let service = service.clone();
                let pairing = pairing.clone();
                tokio::spawn(async move {
                    let handler = service_fn(move |request| {
                        handle(request, service.clone(), pairing.clone())
                    });
                    let _ = hyper::server::conn::http1::Builder::new()
                        .timer(TokioTimer::new())
                        .header_read_timeout(Duration::from_secs(10))
                        .max_buf_size(16 * 1024)
                        .serve_connection(TokioIo::new(stream), handler)
                        .with_upgrades()
                        .await;
                });
            }
            Err(error) => {
                tracing::warn!(%error, "engine listener accept failed");
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

async fn handle(
    mut request: Request<Incoming>,
    service: Arc<dyn roboco_rpc::RpcService>,
    pairing: PairingStore,
) -> Result<Reply, Infallible> {
    // Preserve the native-only local IPC boundary for HTTP as well as WebSocket.
    if request.headers().contains_key("origin") {
        return Ok(reply(
            StatusCode::FORBIDDEN,
            "origin not allowed on local IPC",
        ));
    }
    if request.uri().query().is_some() {
        return Ok(reply(
            StatusCode::BAD_REQUEST,
            "query parameters are not supported",
        ));
    }
    let path = request.uri().path();
    if path == "/pairing/redeem" && request.method() == hyper::Method::POST {
        let Some(code) = bearer(&request).map(str::to_owned) else {
            return Ok(reply(StatusCode::UNAUTHORIZED, "invalid credential"));
        };
        let body = match tokio::time::timeout(
            Duration::from_secs(5),
            Limited::new(request.into_body(), 4096).collect(),
        )
        .await
        {
            Ok(Ok(body)) => body.to_bytes(),
            _ => return Ok(reply(StatusCode::BAD_REQUEST, "invalid request body")),
        };
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Redemption {
            #[serde(default)]
            label: String,
        }
        let Ok(input) = serde_json::from_slice::<Redemption>(&body) else {
            return Ok(reply(StatusCode::BAD_REQUEST, "expected device label"));
        };
        let result = tokio::task::spawn_blocking(move || pairing.redeem(&code, &input.label)).await;
        return Ok(match result {
            Ok(Ok(Some(grant))) => json(StatusCode::OK, &grant),
            Ok(Ok(None)) => reply(StatusCode::UNAUTHORIZED, "invalid credential"),
            _ => reply(StatusCode::BAD_REQUEST, "redemption failed"),
        });
    }
    if path == "/pairing/session" && request.method() == hyper::Method::GET {
        let Some(token) = bearer(&request).map(str::to_owned) else {
            return Ok(reply(StatusCode::UNAUTHORIZED, "invalid credential"));
        };
        let result = tokio::task::spawn_blocking(move || pairing.authenticate(&token)).await;
        return Ok(match result {
            Ok(Ok(Some(session))) => json(StatusCode::OK, &session),
            Ok(Ok(None)) => reply(StatusCode::UNAUTHORIZED, "invalid credential"),
            _ => reply(StatusCode::INTERNAL_SERVER_ERROR, "session check failed"),
        });
    }
    if path == "/health" && request.method() == hyper::Method::GET {
        return Ok(json(StatusCode::OK, &serde_json::json!({"status":"ok"})));
    }
    if path != "/" || request.method() != hyper::Method::GET {
        return Ok(reply(StatusCode::NOT_FOUND, "not found"));
    }
    let upgrade = request
        .headers()
        .get("upgrade")
        .and_then(|v| v.to_str().ok());
    let connection = request
        .headers()
        .get("connection")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    let version = request
        .headers()
        .get("sec-websocket-version")
        .and_then(|v| v.to_str().ok());
    let key = request
        .headers()
        .get("sec-websocket-key")
        .and_then(|v| v.to_str().ok());
    if !upgrade.is_some_and(|v| v.eq_ignore_ascii_case("websocket"))
        || !connection
            .split(',')
            .any(|v| v.trim().eq_ignore_ascii_case("upgrade"))
        || version != Some("13")
        || !key.is_some_and(|value| {
            base64::engine::general_purpose::STANDARD
                .decode(value)
                .is_ok_and(|bytes| bytes.len() == 16)
        })
    {
        return Ok(reply(StatusCode::BAD_REQUEST, "expected WebSocket upgrade"));
    }
    let accept = derive_accept_key(key.unwrap().as_bytes());
    let upgraded = hyper::upgrade::on(&mut request);
    tokio::spawn(async move {
        if let Ok(io) = upgraded.await {
            let ws = tokio_tungstenite::WebSocketStream::from_raw_socket(
                TokioIo::new(io),
                Role::Server,
                None,
            )
            .await;
            roboco_rpc::serve_websocket(ws, service).await;
        }
    });
    Ok(Response::builder()
        .status(StatusCode::SWITCHING_PROTOCOLS)
        .header("upgrade", "websocket")
        .header("connection", "Upgrade")
        .header("sec-websocket-accept", accept)
        .body(Full::new(Bytes::new()))
        .unwrap())
}

fn bearer(request: &Request<Incoming>) -> Option<&str> {
    if request.headers().get_all("authorization").iter().count() != 1 {
        return None;
    }
    request
        .headers()
        .get("authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}
fn reply(status: StatusCode, body: &'static str) -> Reply {
    Response::builder()
        .status(status)
        .header("cache-control", "no-store")
        .body(Full::new(Bytes::from_static(body.as_bytes())))
        .unwrap()
}
fn json(status: StatusCode, value: &impl serde::Serialize) -> Reply {
    Response::builder()
        .status(status)
        .header("cache-control", "no-store")
        .header("content-type", "application/json")
        .body(Full::new(Bytes::from(
            serde_json::to_vec(value).expect("serializable response"),
        )))
        .unwrap()
}
