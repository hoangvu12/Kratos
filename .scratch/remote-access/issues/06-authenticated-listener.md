# 06 — Authenticated remote listener

**What to build:** One listener, two policies: loopback stays credential-free exactly as today, while an opt-in additional bind (LAN) requires a valid session on every dial — HTTP routes and WebSocket upgrades alike. Authenticated remote clients enjoy full RPC parity with the local window. No TLS ever; tunnels are the operator's internet path.

**Blocked by:** 05 — Pairing store: codes, sessions, redemption, revocation CLI.

**Status:** ready-for-human

**Parent:** `.scratch/remote-access/spec.md`

- [x] Unauthenticated dials on the remote bind are rejected; loopback needs no credential
- [x] A session-authenticated test client exercises the full RPC surface (transcripts, files, terminal, queue) against a spawned engine
- [x] Revoking a session cuts off its next connection attempt
- [x] The engine logs which bind(s) it is serving at startup

## Validation

Implemented in `6c42d324`. `cargo test --locked -p roboco-engine --test remote_listener --test pairing` passed all 3 tests. A real engine on the paired listener rejects unauthenticated/wrong/revoked sessions, serves file/transcript/queue/terminal RPCs, and closes existing connections when the listener is dropped. The listener policy belongs to the bind and cannot be bypassed by dialing its port from loopback. No TLS is implemented. Ticket 07 supplies user-facing configuration and startup binding.
