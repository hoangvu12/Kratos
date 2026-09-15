# 04 — Remove WorkOS auth + sign-in UI

**What to build:** Every account surface disappears: the WorkOS auth state machine and its routes, login/logout CLI flows, org onboarding, and signed-out/needs-org UI states. The app boots straight into local mode with no credential flow anywhere.

**Blocked by:** 03 — Remove sync room clients; state feeds go engine-local.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [x] WorkOS client-id/token/refresh machinery and env vars are gone
- [x] No sign-in, org, or signed-out surface remains in the UI
- [x] App starts directly into the local engine session; CLI auth subcommands removed
- [ ] Workspace tests and UI regression suite green

## Implementation notes

Removed WorkOS service/state, app-account RPCs, CLI login/logout, organization
onboarding and sync-switch UI. Desktop and headless boot directly into the local
engine profile. Provider AgentAccounts and their OAuth flows remain available.
The existing deferred engine listener and instance lock still coordinate multiple
local windows; engine readiness now determines the shell gate.

Validation in progress: proto 23 tests and RPC 6 tests passed. UI and desktop binary
`cargo check --tests` passed on Windows. Full UI library regressions are running;
record final result before resolving the ticket. Repository-wide format check has
pre-existing differences; touched files have been formatted separately.
