# 07 — Engine-side remote Settings + `--network` + startup URL

**What to build:** The user-facing surfaces that control ticket 06's listener: a Settings page (remote-access toggle, paired-session list with revoke), a `--network`/env override for headless engines (Settings file remains what the desktop app edits; the engine states which mechanism it used), and a startup print of a fresh pairing URL for headless provisioning.

**Blocked by:** 06 — Authenticated remote listener.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] Flipping the toggle or flag changes whether the remote bind exists, without restart surprises
- [ ] Settings lists sessions and revocation takes effect from the UI
- [ ] A headless engine start prints a usable pairing URL (credential in fragment)
- [ ] Flag/file disagreement resolves safely (loopback-only) and is logged
