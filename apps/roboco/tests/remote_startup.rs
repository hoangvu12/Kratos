//! Headless provisioning prints a redeemable fragment URL from the actual process.
use serde_json::{Value, json};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    time::Duration,
};

struct Child(std::process::Child);
impl Drop for Child {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[tokio::test]
async fn headless_network_flag_prints_a_usable_pairing_url() {
    let dir = tempfile::tempdir().unwrap();
    let mut child = Child(
        Command::new(env!("CARGO_BIN_EXE_roboco"))
            .args(["headless", "--network", "--network-address", "127.0.0.1:0"])
            .env("ROBOCO_DATA_DIR", dir.path())
            .env("ROBOCO_IPC_PORT", "0")
            .env("ROBOCO_HARNESS", "mock")
            .env_remove("ROBOCO_NETWORK")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let stdout = child.0.stdout.take().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if let Some(url) = line.strip_prefix("Pairing URL: ") {
                let _ = tx.send(url.to_owned());
            }
        }
    });
    let url = reqwest::Url::parse(
        &rx.recv_timeout(Duration::from_secs(30))
            .expect("startup pairing URL"),
    )
    .unwrap();
    let code = url.fragment().unwrap().strip_prefix("token=").unwrap();
    let base = format!("http://{}:{}", url.host_str().unwrap(), url.port().unwrap());
    let response = reqwest::Client::new()
        .post(format!("{base}/pairing/redeem"))
        .bearer_auth(code)
        .json(&json!({"label":"Startup client"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let grant: Value = response.json().await.unwrap();
    let rpc = roboco_rpc::connect_ws_authenticated(
        &base.replacen("http", "ws", 1),
        grant["credential"].as_str().unwrap(),
    )
    .await
    .unwrap();
    assert!(
        rpc.call(roboco_rpc::methods::ENGINE_INFO, json!({}))
            .await
            .is_ok()
    );
    drop(rpc);
    drop(child);
    reader.join().unwrap();
}
