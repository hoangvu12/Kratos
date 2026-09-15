# t3code auth research — could its model replace WorkOS in roboco?

Research date: 2026-09-15. Source: `pingdotgg/t3code` @ `main` (shallow clone, since deleted).
Citations are `path:line` against that clone. Companion reading: roboco `edge/src/index.ts`,
`crates/engine/src/auth.rs` (both cited as roboco paths).

## TL;DR

t3code (the coding agent at t3.codes) has **no billing and no entitlement system tied to money**.
It is a local-first, no-account-required app; the entire cloud surface ("T3 Connect") is an
opt-in relay for tunnels/notifications built on **Clerk** for identity only. "Entitlement" is a
single relay-enforced quota — max 3 managed tunnels per user, overridable by a manual DB row.
The model that maps to roboco is: **IdP-verified JWTs in, edge-issued short-lived scoped tokens
out, per-user resources, quotas as plain DB rows.**

## What t3code actually does

### 1. Two separate trust boundaries (this is the core idea)

- **Environment auth** (the local server a user runs — analogous to roboco's Rust engine):
  the environment issues its *own* scoped sessions (browser cookies, bearer tokens, DPoP-bound
  tokens) and enforces capabilities itself. "Cloud identity and relay credentials belong to a
  separate trust boundary… A relay token is never an environment login."
  (`docs/internals/environment-auth.md:1-5`)
- **Cloud auth** (T3 Connect relay — analogous to roboco's edge Worker): Clerk identity +
  relay-issued DPoP access tokens. The relay is a "trusted broker" that mints one-time
  bootstrap credentials; it never proxies app traffic.
  (`docs/internals/t3-connect.md:3-6`, `:11-18`)

### 2. Sign-in mechanism: Clerk, three client flows

All flows use the same Clerk application; interactive clients use a JWT template
(`t3-relay`, aud `t3-code-relay`), the CLI is a separate public OAuth app.

- **Web/mobile**: Clerk SDK (`useAuth().getToken({jwtTemplate})`) —
  `apps/web/src/cloud/managedAuth.tsx:40`, `:86`; `apps/web/src/cloud/publicConfig.ts:77-83`;
  template config in `docs/operations/connect-setup.md:50-61`.
- **Desktop**: Clerk Electron native API with deep-link redirects `t3code://app/`
  (`docs/operations/connect-setup.md:63-76`).
- **CLI, headed**: public OAuth client, **PKCE + loopback** redirect
  `http://127.0.0.1:34338/callback`. It routes through a hosted `/connect` page first because
  Clerk's sign-in redirect loses authorize params (`packages/shared/src/connectAuth.ts:34-60`,
  `:89-106`; `apps/server/src/cloud/CliTokenManager.ts:459-526`;
  `docs/internals/t3-connect.md:70-79`; setup at `docs/operations/connect-setup.md:37-48`).
  No client secret is stored (`docs/internals/t3-connect.md:73`).
- **CLI, headless/SSH**: **OAuth device authorization grant (RFC 8628)** — polls Clerk's token
  endpoint directly while the user approves a short code on Clerk's hosted device page; no
  redirect URI, no hosted-app involvement (`apps/server/src/cloud/CliTokenManager.ts:42`,
  `:327-416`; `docs/internals/t3-connect.md:81-86`).

### 3. Client-side token storage & refresh (CLI)

- `PersistedToken { accessToken, refreshToken, expiresAtEpochMs, identity }`
  (`apps/server/src/cloud/CliTokenManager.ts:121-127`) stored as JSON under secret key
  `cloud-cli-oauth-token` (`:39`, `:431-445`) in the **ServerSecretStore**: a 0700 directory
  of 0600 files with atomic tmp+rename writes
  (`apps/server/src/auth/ServerSecretStore.ts:159-170`, `:188-223`).
- Refresh: `refresh_token` grant **directly against Clerk**, 5 minutes early, single-flight
  via semaphore (`CliTokenManager.ts:41`, `:447-457`, `:528-543`). An unreadable/revoked stored
  credential falls through to fresh login rather than dead-ending (`:550-565`).

### 4. Server-side identity validation (the relay Worker)

- Session JWTs: `verifyToken(token, {secretKey, audience})` from `@clerk/backend`
  (`infra/relay/src/http/Api.ts:1239-1255`).
- CLI OAuth tokens: fallback via `createClerkClient().authenticateRequest(..., {acceptsToken:
  "oauth_token"})` (`Api.ts:1257-1281`), chained in `verifyRelayClientBearerToken`
  (`Api.ts:1283-1299`). The middleware stamps a `RelayClientPrincipal {userId}` into the
  request context (`Api.ts:236-273`). Config keys: `CLERK_SECRET_KEY`,
  `CLERK_PUBLISHABLE_KEY`, `CLERK_JWT_AUDIENCE` (`infra/relay/src/worker.ts:168-196`).
- **Token exchange**: interactive clients swap the Clerk JWT (`subject_token`) plus a DPoP
  proof for a **relay-issued 30-minute EdDSA JWT** bound to the client's key (`cnf.jkt`) with
  explicit scopes (`Api.ts:705-765`; `infra/relay/src/auth/RelayTokens.ts:26-58`, `:168-216`;
  client side `packages/client-runtime/src/relay/managedRelay.ts:484-525`). Ed25519 signing
  via `jose` (`packages/shared/src/relayJwt.ts:58-80`).
- Long-lived tokens never appear in socket URLs: environments hand out **short-lived WebSocket
  tickets over authenticated HTTP** (`docs/internals/environment-auth.md:26-31`).

### 5. Subscription / entitlement

- **None.** No Stripe/Polar/LemonSqueezy anywhere in the repo (searched; zero hits outside
  vendored `.repos/` docs). The product is free/open-source; users bring their own provider
  subscriptions — "an open source 'bring-your-own-subscription' alternative"
  (`AGENTS.md:5`). No pricing on the marketing site.
- The only quota: **managed tunnel limit, default 3 per user**, enforced at link time with a
  per-user override table `relay_managed_tunnel_limits` — manual operator grants, not linked
  to any payment (`infra/relay/src/environments/ManagedTunnelLimits.ts:14-18`, `:60-121`).
  Link-challenge JWTs also carry feature flags: `notificationsEnabled`,
  `liveActivitiesEnabled`, `managedTunnelsEnabled` (`RelayTokens.ts:39-42`, `:153-163`).

### 6. Orgs / multi-user

- **No orgs.** Identity is a flat Clerk `sub`; resources are per-user × per-environment.
  "Managed allocations belong to a user/environment pair" (`docs/internals/t3-connect.md:48`).
  Sharing = linking multiple clients to one environment (the machine's server), not team
  workspaces.

### 7. Local-only / no-account mode

- The default. The app is fully functional with no cloud config — "T3 Connect is disabled in
  a fresh clone" (`docs/operations/connect-setup.md:9`). Local pairing uses a startup URL with
  the secret in the **URL fragment** so it never reaches the hosted origin
  (`docs/internals/remote.md:34-38`); dev credentials are seeded per-environment
  (`docs/internals/environment-auth.md:40-57`).

## Applicability to roboco

roboco today: WorkOS AuthKit public client (Rust engine builds the authorize URL), edge-held
code exchange + refresh (`edge/src/index.ts:9-13`, `:156-159`), org-scoped JWT `?token=` auth
for WS Durable Object rooms (`edge/src/index.ts:161-162`, `:246-256`), engine state machine
`SignedOut | NeedsOrganization | SignedIn{user, orgId}` with edge `/auth/exchange`,
`/auth/refresh`, `/auth/orgs` (`crates/engine/src/auth.rs:64-74`, `:584-639`, `:674-770`).

What the t3code model changes:

- **Routes replaced**: `/auth/exchange` and `/auth/refresh` disappear — the engine talks
  OAuth directly to the IdP (PKCE loopback + device-code headless already exist in spirit:
  roboco has a loopback server `auth.rs:831-849` and a paste-code flow `auth.rs:448-478`;
  t3code's device grant is the cleaner headless replacement). The edge keeps only
  `GET /.well-known/jwks`-style verification (or a secret-key verify) plus a token-exchange
  endpoint. `/auth/orgs` + `POST /auth/orgs` are deleted along with the org concept.
- **What the edge needs**: IdP JWT verification (JWKS fetch + cache for Clerk session JWTs;
  `@clerk/backend` runs fine in a Worker), an Ed25519 keypair for issuing its own short-lived
  scoped tokens, and a KV/DO table for quotas if entitlements are ever needed. WorkOS's
  refresh-token rotation and org-scope refresh (`auth.rs:532-548`) go away entirely —
  Clerk refresh tokens are not single-use-rotated per exchange, so the single-flight
  `refresh_gate` (`auth.rs:223-225`) becomes a plain cache.
- **Rooms**: roboco's rooms are already per-user-derived (`ws4/{orgId}/{userId}`,
  `edge/src/index.ts:256`) — the org check is just a membership gate (`:248`, `:309`).
  Dropping orgs means `auth.orgId` checks become a no-op and `NeedsOrganization` collapses.
  Adopt t3code's rule that long-lived tokens stay out of WS URLs: issue short-lived room
  tickets over authenticated HTTP (`environment-auth.md:26-31`) instead of the current
  long-lived `?token=` JWT.
- **Rust engine migration**: `AuthConfig.workos_*` → IdP endpoints + client id;
  `StoredSession` shape is already `{refresh_token, user, org_id}` (`auth.rs:155-162`) and
  maps 1:1 to t3code's `PersistedToken`; `state_for`'s org gate (`auth.rs:858-868`) is
  removed or replaced by an entitlement check against the edge.

## Candidate designs for roboco, ranked

1. **Clerk identity + edge-issued short-lived tokens (direct port of t3code).** Engine does
   PKCE/device-code OAuth with Clerk; edge verifies Clerk JWTs, issues 30-min EdDSA room
   tokens (optionally DPoP-bound); orgs dropped in favor of per-user rooms; any future
   entitlement is a DO/KV row like `ManagedTunnelLimits`. Best fit: it deletes the most code
   (edge exchange/refresh/orgs, engine org state, refresh rotation) and matches roboco's
   already per-user room topology. Cost: Clerk account + new headless device-grant flow in Rust.
2. **No-account-first, opt-in cloud (t3code's product philosophy, any IdP).** Default is an
   anonymous device identity (rooms keyed by device/user id minted locally); sign in only to
   enable sync/relay features. Least auth surface and best local-first fit, but changes
   product behavior (currently every session is WorkOS-backed) and room ownership/claiming
   needs a migration story.
3. **Keep the WorkOS-shaped edge broker, swap provider + add a local quota table.** Minimal
   Rust churn (same state machine, orgs become static), edge still holds client secrets and
   does exchange/refresh — but this keeps the single-use-refresh-token race machinery and
   the WorkOS coupling t3code avoids; only worth it if a hard requirement for org gating or
   MFA/SSO features survives.
