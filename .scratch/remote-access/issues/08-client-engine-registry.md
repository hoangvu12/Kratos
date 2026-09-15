# 08 — Client engine registry + merged sidebar

**What to build:** The shell stops assuming one engine connection. A registry holds the local engine plus every paired remote (pairing URL paste-in for the first version), each with its own supervised connection that auto-connects at startup and reconnects with capped backoff. Sessions from all engines appear in the existing sidebar — same rows, same `@ device` tags, same device grouping — with connection-status dots on engine surfaces. No new sidebar components.

**Blocked by:** 06 — Authenticated remote listener.

**Status:** implemented

**Parent:** `.scratch/remote-access/spec.md`

- [x] Adding a remote engine by URL connects it; its sessions appear beside local ones under its device label
- [x] Removing/forgetting an engine drops only its rows
- [x] Killing a remote engine marks it reconnecting; recovery restores its rows without app restart
- [x] Registry behavior verified by tests against real engines on loopback ports (spec's seam)

## Implementation and evidence

- GPUI-free `engine_registry` owns paired credentials, supervised authenticated sockets, capped reconnect with health checks, engine-scoped identities, and raw per-engine snapshots. Existing sidebar feeds receive merged projected rows.
- Devices settings accepts a pairing URL, shows connection state, renames through the owning engine, and forgets only that engine. Cached rows survive disconnect and application restart. Damaged saved pairing configuration preserves the file and local access.
- `cargo check -p roboco-ui --tests --offline` passed.
- `cargo test -p roboco-ui --lib engine_registry::tests --offline -- --test-threads=1`: **3 passed**. Tests use two real EngineCore instances and authenticated loopback listeners; stop and reassemble the remote engine on the same disk and port; verify reconnect, preserved rows, persistent pairing reload, offline refusal, forget, malformed configuration recovery, and ID collision safety.
- Transcript device/subagent-reference projection helper added for ticket10 integration; final combined branch validation covers its consumer wiring.
