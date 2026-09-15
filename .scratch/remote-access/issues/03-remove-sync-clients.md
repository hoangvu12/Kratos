# 03 — Remove sync room clients; state feeds go engine-local

**What to build:** The sync-era machinery that pushed sessions/devices through the cloud is removed: the chat/registry room clients, the device-room relay client, and their tests. Client state (devices, spaces, `@ device` tags, session lists) now comes solely from the local engine's RPCs. The app behaves as a single-engine client, fully working locally.

**Blocked by:** None — can start immediately.

**Status:** resolved

**Implementation:** merged; integrated headless UI validation completed on the PR branch.

**Parent:** `.scratch/remote-access/spec.md`

- [x] Room/relay sync clients and their integration tests are deleted; local loro doc storage stays
- [x] Sidebar, spaces, and device vocabulary render correctly from local-engine data alone
- [x] No sign-in or cloud path is required for any local behavior
- [x] Headless UI layout regression suite stays green


## Comments

2026-09-15: Engine-local cutover merged on main (`f3a322a8`, with relay removal `50086d84`). Chat and registry room clients, network transport helpers, relay integration tests, and cloud diff publication are removed. Local document snapshots preserve existing Loro lineages and recover pending legacy outbox updates. Completed tool/subagent sidecars persist in the engine's SQLite store. Device/chat/space/session feeds filter legacy foreign-device rows, and registry writes compact immediately without awaiting cloud acknowledgements.

Validation on the implementation branch:

- `cargo test -p roboco-engine --lib --test engine_local --test e2e --test message_queue --test restart_resume --offline`: 220 passed, 2 intentionally ignored real-agent tests.
- `cargo test -p roboco-doc engine_local_commits --offline`: passed; persisted deletes survive restart and stale updates.
- The new real-WebSocket listener test checks local device/chat feeds, exclusion of old cloud rows, transcript restart persistence, and local sidecar persistence.
- Integrated headless UI layout validation is pending alongside ticket 04; keep the remaining acceptance checks open until that run completes.
