# 07 — Engine-side remote Settings + `--network` + startup URL

**What to build:** The user-facing surfaces that control ticket 06's listener: a Settings page (remote-access toggle, paired-session list with revoke), a `--network`/env override for headless engines (Settings file remains what the desktop app edits; the engine states which mechanism it used), and a startup print of a fresh pairing URL for headless provisioning.

**Blocked by:** 06 — Authenticated remote listener.

**Status:** ready-for-human

**Parent:** `.scratch/remote-access/spec.md`

- [x] Flipping the toggle or flag changes whether the remote bind exists, without restart surprises
- [x] Settings lists sessions and revocation takes effect from the UI
- [x] A headless engine start prints a usable pairing URL (credential in fragment)
- [x] Flag/file disagreement resolves safely (loopback-only) and is logged

## Implementation and validation

Remote access Settings controls the live listener, creates/copies pairing links,
and lists/revokes sessions through engine RPCs. Saved settings, headless
`--network`/`ROBOCO_NETWORK`, custom bind addresses, and public tunnel URLs are
documented in `docs/reference/remote-access.md`. Conflicting or invalid settings
fail closed and log their source. Wildcard binds advertise a LAN address.

Passed on Windows:
- `cargo check --locked -p roboco`.
- `cargo test --locked -p roboco-engine --test remote_settings`: two real-listener
  tests, including toggle/rebind, pairing/revoke, self-disabling remote callers,
  malformed settings, and flag/file conflicts.
- `cargo test --locked -p roboco --test remote_startup`: actual headless process
  prints a URL that redeems and opens authenticated engine RPC.
- `git diff --check`.
