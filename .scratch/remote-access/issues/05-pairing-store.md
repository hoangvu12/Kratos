# 05 — Pairing store: codes, sessions, redemption, revocation CLI

**What to build:** The engine's credential foundation: short-lived single-use pair codes minted from the OS CSPRNG, a redeem route that trades a pairing URL (credential in the `#fragment`) for a non-expiring session, verifiers stored hashed at rest, and a second-process CLI (`roboco engine pairing create/list/revoke`) that works against a running engine's database without coordination. No scopes, no renewal machinery.

**Blocked by:** 04 — Remove WorkOS auth + sign-in UI.

**Status:** ready-for-human

**Parent:** `.scratch/remote-access/spec.md`

- [x] A minted pairing URL redeems exactly once, then is worthless; expired/unknown codes fail closed
- [x] Sessions do not expire; `list` shows them with labels and last-seen; `revoke` works while the engine runs
- [x] Stored credentials are hashed; the database file alone yields no usable tokens
- [x] All behavior verified by tests driving a spawned engine over its listener (the spec's single seam)

## Implementation and validation

PairingStore uses 32-byte OS-random credentials and domain-separated SHA-256
verifiers in an engine-local SQLite database. Redemption consumes the code and
issues a session in one transaction. The CLI opens this database without taking
the engine instance lock. Pairing HTTP and local WebSocket RPC share one port.

Passed on Windows:
- `cargo test -p roboco-engine --test pairing`: two real-engine listener tests,
  covering concurrent single-use redemption, expiry, restart persistence,
  last-seen, revocation, hash-at-rest inspection, and local WebSocket RPC.
- `cargo test --locked -p roboco --test pairing_cli`: the actual CLI in a second
  process creates, lists, and revokes credentials while the engine runs.
- `git diff --check`.

Remote bind authorization is ticket 06; this ticket adds the shared listener
foundation and local redemption route.
