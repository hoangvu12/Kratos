use std::sync::Arc;

use roboco_engine::{EngineCore, EngineProfile, HarnessRegistry, pairing::PairingStore};
use roboco_rpc::{connect_ws_authenticated, methods};
use serde_json::{Value, json};

#[tokio::test]
async fn remote_manual_paths_are_checked_and_created_on_the_engine() {
    let dir = tempfile::tempdir().unwrap();
    let core = EngineCore::assemble_with_profile(
        EngineProfile::local(dir.path()).unwrap(),
        Arc::new(HarnessRegistry::new()),
        roboco_engine::HarnessId::Mock,
    )
    .unwrap();
    let server = roboco_engine::serve_engine_remote(
        "127.0.0.1:0".parse().unwrap(),
        core.rpc_service(),
        dir.path(),
    )
    .await
    .unwrap();
    let store = PairingStore::open(dir.path()).unwrap();
    let code = store.create_code("path test", 300).unwrap();
    let grant: Value = reqwest::Client::new()
        .post(format!("http://{}/pairing/redeem", server.address))
        .bearer_auth(code.credential)
        .json(&json!({"label":"client"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let rpc = connect_ws_authenticated(
        &format!("ws://{}", server.address),
        grant["credential"].as_str().unwrap(),
    )
    .await
    .unwrap();
    let path = dir.path().join("new-project");
    let inspected = rpc
        .call(methods::PREPARE_SPACE_PATH, json!({"path":path}))
        .await
        .unwrap();
    assert_eq!(inspected["exists"], false);
    assert!(!path.exists());
    let prepared = rpc
        .call(
            methods::PREPARE_SPACE_PATH,
            json!({"path":path,"createIfMissing":true}),
        )
        .await
        .unwrap();
    assert_eq!(prepared["exists"], true);
    assert!(path.is_dir());
    rpc.call(methods::MUTATE, json!({"op":"createSpace","spaceId":"manual","deviceId":core.device_id,"path":prepared["path"]})).await.unwrap();
    let mut spaces = rpc
        .subscribe(methods::WATCH_SPACES, json!({}))
        .await
        .unwrap();
    let rows = tokio::time::timeout(std::time::Duration::from_secs(5), spaces.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        rows.as_array()
            .unwrap()
            .iter()
            .any(|row| row["id"] == "manual" && row["path"] == prepared["path"])
    );
    std::fs::create_dir(path.join(".hidden")).unwrap();
    std::fs::create_dir(path.join("visible")).unwrap();
    let listing = rpc
        .call(methods::LIST_FOLDERS, json!({"path":path}))
        .await
        .unwrap();
    assert!(
        !listing["entries"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["name"] == ".hidden")
    );
    let listing = rpc
        .call(methods::LIST_FOLDERS, json!({"path":path,"query":"."}))
        .await
        .unwrap();
    assert!(
        listing["entries"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["name"] == ".hidden")
    );
    #[cfg(windows)]
    let foreign = "/tmp/foreign-unix-project";
    #[cfg(not(windows))]
    let foreign = "C:\\foreign-windows-project";
    assert!(
        rpc.call(
            methods::PREPARE_SPACE_PATH,
            json!({"path":foreign,"createIfMissing":true})
        )
        .await
        .is_err()
    );
    let file = path.join("file.txt");
    std::fs::write(&file, "file").unwrap();
    assert!(
        rpc.call(
            methods::PREPARE_SPACE_PATH,
            json!({"path":file,"createIfMissing":true})
        )
        .await
        .is_err()
    );
    drop(spaces);
    drop(rpc);
    drop(server);
    core.shutdown().await;
}
