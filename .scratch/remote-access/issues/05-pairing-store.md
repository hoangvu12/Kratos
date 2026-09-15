# 05 — Pairing store: codes, sessions, redemption, revocation CLI

**What to build:** The engine's credential foundation: short-lived single-use pair codes minted from the OS CSPRNG, a redeem route that trades a pairing URL (credential in the `#fragment`) for a non-expiring session, verifiers stored hashed at rest, and a second-process CLI (`roboco engine pairing create/list/revoke`) that works against a running engine's database without coordination. No scopes, no renewal machinery.

**Blocked by:** 04 — Remove WorkOS auth + sign-in UI.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] A minted pairing URL redeems exactly once, then is worthless; expired/unknown codes fail closed
- [ ] Sessions do not expire; `list` shows them with labels and last-seen; `revoke` works while the engine runs
- [ ] Stored credentials are hashed; the database file alone yields no usable tokens
- [ ] All behavior verified by tests driving a spawned engine over its listener (the spec's single seam)
