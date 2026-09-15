use super::*;
use gpui::AppContext;
use std::time::{Duration, Instant};

fn wait_for(
    cx: &mut gpui::TestAppContext,
    state: &Entity<AppState>,
    description: &str,
    predicate: impl Fn(&AppState) -> bool,
) {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        cx.run_until_parked();
        if state.read_with(cx, |state, _| predicate(state)) {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "Timed out waiting for {description}"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[gpui::test]
fn paired_history_survives_disconnect_and_client_restart(cx: &mut gpui::TestAppContext) {
    // Real engines over real sockets: RPC completions wake gpui tasks from
    // tokio workers, so this test intentionally mixes I/O with the
    // deterministic scheduler (the sanctioned `allow_parking` escape).
    cx.executor().allow_parking();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    cx.update(|cx| gpui_tokio::init_from_handle(cx, runtime.handle().clone()));
    let local_dir = tempfile::tempdir().unwrap();
    let remote_dir = tempfile::tempdir().unwrap();
    let (core, listener, pairing_url) = runtime.block_on(async {
        let core = roboco_engine::EngineCore::assemble_with_profile(
            roboco_engine::EngineProfile::local(remote_dir.path()).unwrap(),
            Arc::new(roboco_engine::default_registry()),
            HarnessId::Mock,
        )
        .unwrap();
        let listener = roboco_engine::serve_engine_remote(
            "127.0.0.1:0".parse().unwrap(),
            core.rpc_service(),
            remote_dir.path(),
        )
        .await
        .unwrap();
        let code = roboco_engine::pairing::PairingStore::open(remote_dir.path())
            .unwrap()
            .create_code("history test", 300)
            .unwrap();
        let url = roboco_engine::pairing::pairing_url(
            &format!("http://{}", listener.address),
            &code.credential,
        )
        .unwrap();
        (core, listener, url)
    });
    let port = runtime.block_on(async {
        tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    });
    let config = EngineBootConfig {
        data_dir: local_dir.path().to_path_buf(),
        ipc_port: port,
        default_harness: HarnessId::Mock,
    };
    let state = cx.new(|_| AppState::new());
    cx.update(|cx| AppState::bootstrap(state.clone(), config.clone(), cx));
    wait_for(cx, &state, "client registry", |s| s.registry().is_some());
    let registry = state.read_with(cx, |s, _| s.registry().unwrap().clone());
    let key = runtime
        .block_on(registry.pair(&pairing_url, "Remote history"))
        .unwrap();
    let target = registry.target(&key).unwrap();
    wait_for(cx, &state, "paired engine", |_| target.is_connected());
    let chat = ScopedId::encode(&key, "cached-chat");
    runtime.block_on(async {
        target
            .call(
                methods::MUTATE,
                serde_json::json!({
                    "op":"createChat","chatId":chat,
                    "deviceId":ScopedId::encode(&key, &core.device_id)
                }),
            )
            .await
            .unwrap();
        target
            .call(
                methods::QUEUE_MESSAGE,
                serde_json::json!({
                    "chatId":chat,"text":"Keep this remote history after reconnect"
                }),
            )
            .await
            .unwrap();
    });
    wait_for(cx, &state, "remote chat in sidebar", |s| {
        s.chats.iter().any(|c| c.id == chat)
    });
    state.update(cx, |s, cx| s.select_chat(Some(chat.clone()), cx));
    wait_for(cx, &state, "remote transcript", |s| {
        !s.transcript.is_empty()
    });
    let entries = state.read_with(cx, |s, _| s.transcript.clone());
    let cache = crate::engine_cache::EngineCache::new(local_dir.path());
    wait_for(cx, &state, "transcript saved", |_| {
        cache
            .load_transcript(&key.0, "cached-chat")
            .is_some_and(|rows| !rows.is_empty())
    });
    drop(listener);
    wait_for(cx, &state, "remote offline", |_| !target.is_connected());
    state.read_with(cx, |s, _| {
        assert_eq!(s.selected_chat.as_deref(), Some(chat.as_str()));
        assert!(!s.transcript.is_empty());
        assert!(s.chats.iter().any(|row| row.id == chat));
    });
    // A fresh client entity must load persisted sidebar rows and the selected
    // transcript even though no remote socket can supply a reset frame.
    drop(state);
    runtime.block_on(registry.shutdown());
    drop(registry);
    let reopened = cx.new(|_| AppState::new());
    cx.update(|cx| AppState::bootstrap(reopened.clone(), config, cx));
    wait_for(cx, &reopened, "cached sidebar after restart", |s| {
        s.chats.iter().any(|c| c.id == chat)
    });
    reopened.update(cx, |s, cx| s.select_chat(Some(chat.clone()), cx));
    wait_for(cx, &reopened, "cached transcript after restart", |s| {
        !s.transcript.is_empty()
    });
    reopened.read_with(cx, |s, _| {
        assert_eq!(s.transcript[0].id, entries[0].id);
        assert!(!s.target_for_id(&chat).unwrap().is_connected());
    });
    let registry = reopened.read_with(cx, |s, _| s.registry().unwrap().clone());
    let engine = reopened.read_with(cx, |s, _| s.engine().unwrap().clone());
    drop(reopened);
    runtime.block_on(async {
        registry.shutdown().await;
        engine.shutdown().await;
        core.shutdown().await;
    });
}
