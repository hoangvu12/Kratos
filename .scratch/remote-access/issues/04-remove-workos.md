# 04 — Remove WorkOS auth + sign-in UI

**What to build:** Every account surface disappears: the WorkOS auth state machine and its routes, login/logout CLI flows, org onboarding, and signed-out/needs-org UI states. The app boots straight into local mode with no credential flow anywhere.

**Blocked by:** 03 — Remove sync room clients; state feeds go engine-local.

**Status:** resolved

**Parent:** `.scratch/remote-access/spec.md`

- [x] WorkOS client-id/token/refresh machinery and env vars are gone
- [x] No sign-in, org, or signed-out surface remains in the UI
- [x] App starts directly into the local engine session; CLI auth subcommands removed
- [x] Workspace tests and UI regression suite green

## Implementation notes

Removed WorkOS service/state, app-account RPCs, CLI login/logout, organization
onboarding and sync-switch UI. Desktop and headless boot directly into the local
engine profile. Provider AgentAccounts and their OAuth flows remain available.
The existing deferred engine listener and instance lock still coordinate multiple
local windows; engine readiness now determines the shell gate.

## Validation

- Proto: 23 tests passed. RPC: 6 tests passed.
- UI and desktop binary `cargo check --tests` passed on Windows.
- Rebuilt the UI test executable after engine constructor cleanup and the
  Windows identity-lock fix. All 924 unique nonignored UI tests passed across
  40 module-isolated runs; 5 existing tests were ignored. Verified the union of
  successful/ignored test names covers every test from the executable's `--list`.
- Fixed stale test expectations discovered by this run: the daemon fixture now
  creates an explicit local profile; the Roboco name contains 6 bytes; Windows
  frost tests reflect the renderer's existing frost support. Provider account
  coverage remains included.
- Follow-up (2026-09-15, PR branch): the `0xc0000409` abort was root-caused and
  fixed. The `windows_pulse` waitable-timer thread completes a oneshot that wakes
  a gpui task; under the deterministic test scheduler that is foreign-thread
  scheduling, and a tick landing after a test's scheduler had finished panicked
  inside the oneshot drop path — the double-panic aborted the whole process at
  varying, allocation-heavy-looking points. Tests now take the executor-timer
  path, exactly as non-Windows platforms always do (`crates/ui/src/motion.rs`).
  The full monolithic suite passes repeatedly on Windows (933 passed) and the
  hosted Windows/Linux CI runs green.
- No Linux release/headless UI run was available on this Windows host.
  Workspace-wide checks remain the integration owner's responsibility.
- Repository-wide format check reports pre-existing differences; touched core
  UI/config files were formatted separately. `git diff --check` passed.

Local diagnostic artifacts: `C:/Users/ADMIN/Temp/auth04-ui-final.log`,
`auth04-split/results.json`, and per-module logs beside that JSON. The diagnostic
scripts live outside the repository and are not shipped.
