# 04 — Remove WorkOS auth + sign-in UI

**What to build:** Every account surface disappears: the WorkOS auth state machine and its routes, login/logout CLI flows, org onboarding, and signed-out/needs-org UI states. The app boots straight into local mode with no credential flow anywhere.

**Blocked by:** 03 — Remove sync room clients; state feeds go engine-local.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] WorkOS client-id/token/refresh machinery and env vars are gone
- [ ] No sign-in, org, or signed-out surface remains in the UI
- [ ] App starts directly into the local engine session; CLI auth subcommands removed
- [ ] Workspace tests and UI regression suite green
