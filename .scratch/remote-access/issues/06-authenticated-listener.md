# 06 — Authenticated remote listener

**What to build:** One listener, two policies: loopback stays credential-free exactly as today, while an opt-in additional bind (LAN) requires a valid session on every dial — HTTP routes and WebSocket upgrades alike. Authenticated remote clients enjoy full RPC parity with the local window. No TLS ever; tunnels are the operator's internet path.

**Blocked by:** 05 — Pairing store: codes, sessions, redemption, revocation CLI.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] Unauthenticated dials on the remote bind are rejected; loopback needs no credential
- [ ] A session-authenticated test client exercises the full RPC surface (transcripts, files, terminal, queue) against a spawned engine
- [ ] Revoking a session cuts off its next connection attempt
- [ ] The engine logs which bind(s) it is serving at startup
