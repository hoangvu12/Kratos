//! Pairing through the real engine HTTP/WebSocket listener, with independent
//! SQLite connections providing the same coordination-free CLI boundary.
use roboco_engine::{
    EngineCore, EngineProfile, HarnessRegistry,
    pairing::{PairingStore, pairing_url},
};
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};

async fn engine(dir: &std::path::Path) -> (EngineCore, String, tokio::task::JoinHandle<()>) {
    let core = EngineCore::assemble_with_profile(
        EngineProfile::local(dir).unwrap(),
        Arc::new(HarnessRegistry::new()),
        roboco_engine::HarnessId::Mock,
    )
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(roboco_engine::listener::serve_listener(
        listener,
        core.rpc_service(),
        PairingStore::open(dir).unwrap(),
    ));
    (core, base, task)
}

async fn redeem(client: &reqwest::Client, base: &str, credential: &str) -> reqwest::Response {
    client
        .post(format!("{base}/pairing/redeem"))
        .bearer_auth(credential)
        .json(&json!({"label":"Laptop"}))
        .send()
        .await
        .unwrap()
}

#[tokio::test]
async fn pairing_is_atomic_persistent_revocable_and_hashed() {
    let dir = tempfile::tempdir().unwrap();
    let (core, base, server) = engine(dir.path()).await;
    let client = reqwest::Client::new();
    // Independently opened SQLite connection works despite the engine instance lock.
    let cli_store = PairingStore::open(dir.path()).unwrap();
    let code = cli_store.create_code("test device", 300).unwrap();
    let url = reqwest::Url::parse(&pairing_url(&base, &code.credential).unwrap()).unwrap();
    assert_eq!(url.path(), "/pair");
    assert!(url.query().is_none());
    assert_eq!(
        url.fragment().unwrap(),
        format!("token={}", code.credential)
    );
    let (a, b) = tokio::join!(
        redeem(&client, &base, &code.credential),
        redeem(&client, &base, &code.credential)
    );
    let grant: Value = match (a.status().as_u16(), b.status().as_u16()) {
        (200, 401) => a.json().await.unwrap(),
        (401, 200) => b.json().await.unwrap(),
        statuses => panic!("single use failed: {statuses:?}"),
    };
    let token = grant["credential"].as_str().unwrap();
    let id = grant["session"]["id"].as_str().unwrap();
    assert_eq!(redeem(&client, &base, &code.credential).await.status(), 401);
    assert_eq!(redeem(&client, &base, &"x".repeat(43)).await.status(), 401);
    for entry in std::fs::read_dir(dir.path()).unwrap() {
        let path = entry.unwrap().path();
        if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("pairing.sqlite3")
        {
            let bytes = std::fs::read(path).unwrap();
            for credential in [code.credential.as_str(), token] {
                assert!(
                    !bytes
                        .windows(credential.len())
                        .any(|window| window == credential.as_bytes())
                );
            }
        }
    }
    let before = cli_store.list_sessions().unwrap()[0].last_seen;
    tokio::time::sleep(Duration::from_millis(10)).await;
    let info: Value = client
        .get(format!("{base}/pairing/session"))
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(info["label"], "Laptop");
    assert!(info["lastSeen"].as_i64().unwrap() > before);
    assert!(info.get("credential").is_none());
    // Native local WebSocket RPC remains credential free on the same port.
    let rpc = roboco_rpc::connect_ws(&base.replacen("http", "ws", 1))
        .await
        .unwrap();
    let result = rpc
        .call(roboco_rpc::methods::LIST_HARNESSES, json!({}))
        .await
        .unwrap();
    assert!(result.is_array());
    drop(rpc);
    server.abort();
    core.shutdown().await;
    drop(core);
    let (core, base, server) = engine(dir.path()).await;
    assert_eq!(
        client
            .get(format!("{base}/pairing/session"))
            .bearer_auth(token)
            .send()
            .await
            .unwrap()
            .status(),
        200
    );
    assert!(PairingStore::open(dir.path()).unwrap().revoke(id).unwrap());
    assert_eq!(
        client
            .get(format!("{base}/pairing/session"))
            .bearer_auth(token)
            .send()
            .await
            .unwrap()
            .status(),
        401
    );
    server.abort();
    core.shutdown().await;
}

#[tokio::test]
async fn expiry_and_transport_reject_invalid_credentials() {
    let dir = tempfile::tempdir().unwrap();
    let (core, base, server) = engine(dir.path()).await;
    let store = PairingStore::open(dir.path()).unwrap();
    let code = store.create_code("expires", 1).unwrap();
    let client = reqwest::Client::new();
    tokio::time::sleep(Duration::from_millis(1100)).await;
    assert_eq!(redeem(&client, &base, &code.credential).await.status(), 401);
    assert_eq!(
        client
            .post(format!("{base}/pairing/redeem"))
            .json(&json!({"code":code.credential}))
            .send()
            .await
            .unwrap()
            .status(),
        401
    );
    assert_eq!(
        client
            .post(format!("{base}/pairing/redeem?token={}", code.credential))
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    assert_eq!(
        client
            .get(format!("{base}/health"))
            .header("origin", "https://example.com")
            .send()
            .await
            .unwrap()
            .status(),
        403
    );
    let response = client.get(format!("{base}/health")).send().await.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers()["cache-control"], "no-store");
    server.abort();
    core.shutdown().await;
}
