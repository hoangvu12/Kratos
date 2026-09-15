# Remote access & multi-engine (replacing zeron sync)

Status: ready-for-agent
Spec date: 2026-09-15

## Problem Statement

I run Roboco on Windows and will run agents on a headless Linux server. Today, multi-device access only works through Zeron's cloud: a WorkOS account, an account-scoped edge relay, and CRDT rooms synced via `zeron.sh`. I don't use any of it — it demands a login I don't want, routes my data through someone else's infrastructure, and its UI (sign-in, orgs, signed-out states) clutters the app. Meanwhile the setup I actually want — the t3code experience — is: every machine runs an engine, the engine exposes a link, any client pairs once and never thinks about it again, and one client sees all its engines at once.

## Solution

Roboco drops the account/cloud model entirely and speaks direct. Each engine optionally accepts remote clients over the network (off by default, plain WebSocket, credential required). A client pairs once — by redeeming a short-lived pairing URL minted in Settings, at startup, or from the CLI — and receives a session that does not expire until revoked. The desktop client holds connections to several engines at once (local + remote); their sessions appear in the existing sidebar, separated by the device labels and grouping that already exist, and new chats target whichever engine's space you pick via the existing palette and composer chip. An offline engine's history stays readable from cache behind a thin reconnect indicator. There is no account, no sign-in UI, no cloud, and no cross-engine sync — ever.

## User Stories

1. As the sole user, I want no account or sign-in anywhere in the app, so that nothing stands between me and my local sessions.
2. As a desktop user, I want the app to keep working exactly as today against my local engine, so the rework costs me nothing.
3. As a desktop user, I want to enable remote access on my engine with a Settings toggle, so my other devices can reach it on my LAN.
4. As a server operator, I want to enable remote access with a CLI flag/env on a headless engine, so no GUI is needed to expose it.
5. As a server operator, I want the engine to print a pairing URL at startup, so I can pair a client without extra steps.
6. As a server operator, I want `roboco engine pairing create` to work against a running engine from a second process, so minting a link never requires restarting or SSH-ing into a live session beyond provisioning.
7. As any client, I want to paste a pairing URL once and hold a session that never expires, so pairing is a one-time event per device.
8. As a user, I want the pairing URL to carry its credential in the URL fragment, so the secret never travels over plain HTTP bodies or lands in server logs.
9. As a user, I want every session listed in Settings with device label and last-seen, so I can see what holds access.
10. As a user, I want to revoke any session instantly from Settings or `roboco engine pairing revoke`, so a lost phone stops mattering.
11. As a user, I want pairing codes to be short-lived, single-use, and generated from the OS CSPRNG, so a glimpsed code is worthless.
12. As a user, I want credentials stored hashed at rest, so a stolen database file doesn't leak working tokens.
13. As a client on my LAN, I want to connect with just my session credential over plain WebSocket, so no certificate setup exists.
14. As a remote user over the internet, I want to reach my engine through a tunnel (cloudflared/Tailscale), so TLS and reachability are somebody else's well-tested job.
15. As a desktop user, I want all my paired engines to connect automatically at startup, so my whole fleet is one window.
16. As a desktop user, I want each engine's sessions in the existing sidebar with their `@ device` tags and device grouping, so multi-engine needs no new sidebar UI.
17. As a desktop user, I want an engine's status visible as a dot (connected / reconnecting / off) on its device surfaces, so fleet health is glanceable.
18. As a desktop user, I want to add a project on a specific engine via the existing space palette's device tabs and folder browser, so "add project" works remotely exactly as locally.
19. As a desktop user, I want the composer's space chip to show which engine+folder a new chat will run on, so the target is always visible before I send.
20. As a desktop user, I want to type a path manually and have the engine create the folder if missing, so adding a remote project doesn't require pre-mkdir'd directories.
21. As a desktop user, I want an offline engine's sessions to stay listed and readable from cache, so a sleeping server doesn't blank my sidebar.
22. As a desktop user, I want offline state to be a compact reconnect indicator — not a full-screen banner, so the clean UI is preserved.
23. As a desktop user, I want reconnection to happen automatically with backoff and on app restart, so recovered engines heal without me.
24. As a desktop user, I want writes to an offline engine to be refused rather than queued invisibly, so I always know what actually sent.
25. As a remote client, I want the full RPC surface available — transcripts, terminals, files, queue — so a remote session is first-class, not a reduced mode.
26. As a developer, I want one listener per engine that speaks HTTP (pairing, health) and upgrades to WebSocket (RPC), so there is exactly one network surface to secure and test.
27. As a developer, I want the loopback listener to stay unauthenticated and instance-locked exactly as today, so the local window keeps its zero-friction path.
28. As a user, I want the WorkOS login UI, org onboarding, and signed-out states gone, so no dead account surfaces remain.
29. As a user, I want the iOS app and edge infrastructure removed from the repo, so nothing implies a cloud that no longer exists.
30. As a future contributor, I want upstream Zeron features portable by cherry-pick against a pristine mirror branch, so useful upstream work remains reachable without merges.

## Implementation Decisions

- **Vocabulary** follows the glossary: *engine* (never "environment/server"), *pairing* (never "login"), *session* (the non-expiring client credential). ADRs 0003 (product-not-fork, cherry-pick-only upstream) and 0004 (engine-local data, no cross-engine sync) govern the architecture.
- **Cutover happens first, on main**: delete the edge worker and its room classes, WorkOS auth state machine and routes, the sync room clients and their device-room relay client, sign-in/org UI states, and the iOS app. Local loro doc storage, the engine, and the RPC surface stay. Sync-era concepts (devices, spaces, `@ device` tags) stay in the client state model — their data source changes from sync feeds to engine RPCs.
- **One listener per engine**: the existing loopback WebSocket server grows plain-HTTP routes on the same port (redeem pairing URL → session; health; revocation admin if needed), with the WebSocket upgrade unchanged. A second, opt-in bind address (LAN) serves the same routes with mandatory credentials; loopback stays credential-free and instance-locked as today.
- **Remote access is opt-in**: a Settings-editable remote-access file beside the existing settings, plus `--network`/env override for headless engines; the engine logs which mechanism it used at startup.
- **No TLS on the listener, ever**: plain HTTP/WS on trusted networks; internet exposure via operator-run tunnels. Pairing URLs carry the credential in the `#fragment` so secrets stay out of HTTP request lines and logs.
- **Credential chain, simple variant** (deliberate deviation from t3code/laplus): short-lived single-use pair code/URL → **non-expiring session**; no sliding-renewal machinery, no device tokens, no scopes. Sessions are revocable; revocation takes effect on the next connection attempt (existing connections may be dropped at revocation time). All randomness from the OS CSPRNG; verifiers stored hashed.
- **Pair-code minting works as a second process** against the engine's database (the persisted row is the code's whole existence — the laplus-proven pattern), so CLI minting never coordinates with the running engine.
- **Client engine registry**: the shell holds N supervised engine connections (local first, then paired remotes), each with its own reconnect/backoff state. Session/space/device state merges across engines into the existing sidebar vocabulary; **request routing is per-engine** — every chat, space, and file operation carries the engine identity of its connection.
- **Sidebar, space chip, and space palette remain the existing components**, now fed by the merged multi-engine state; the palette's device tabs enumerate paired engines; the folder browser uses the existing drive/folder listing RPCs against the chosen engine. Manual path entry gains create-if-missing.
- **Offline = cached reads + thin indicator**: per-engine client-side cache of session lists and open transcripts; one compact strip when viewing an offline engine; no offline writes; supervisor retries with capped backoff.
- **Full RPC parity remotely**; the same protocol the local window speaks, just authenticated.
- **Upstream**: a pristine un-renamed Zeron mirror branch is kept in-repo; ports are cherry-picked there first, then carried across the rebrand/removals by hand. No merges into main.

## Testing Decisions

- **One seam: the engine's listener** (HTTP + WebSocket). Tests spawn a real engine on an ephemeral port — mock harness for deterministic agent behavior — and drive it exactly as a client does. No test-only hooks inside the engine, no protocol mocks.
- **Good tests assert external behavior over the listener**: the pairing dance succeeds with a valid URL and fails closed with a wrong/redeemed/expired one; a revoked session is refused on the next dial; an unauthenticated remote dial is rejected while loopback stays open; a second connected client can exercise the same RPC surface as the local one; concurrent clients on different engines route to the right engine.
- **Client-side tests use the same seam**: the engine registry and reconnect/offline logic run against real engines on loopback ports, asserting observable reconnection and cache behavior — not internal state transitions.
- **Prior art**: the existing RPC/engine integration suites (the device-room tests being deleted demonstrate the spawn-and-dial pattern), and the headless UI layout regression suite, which continues unchanged and must stay green through the cutover.

## Out of Scope

- Web/phone client (a later build serves a web UI from the engine)
- Scopes/authorization tiers, sliding sessions, device tokens
- TLS on the listener; any relay or managed-tunnel infrastructure
- Cross-engine sync of sessions, queue, search, or settings (permanently — ADR 0004)
- Composer-level load balancing across engines
- Git-URL/clone as an add-project source
- Any iOS story

## Further Notes

- Research backing: `docs/research/environment-model.md` (t3code/laplus model with citations) and `docs/research/t3code-auth.md` (credential-chain analysis).
- The laplus codebase (not in this repo) is the proven blueprint for the pairing module; port patterns, not files.
- Existing PR #313 to upstream (`windows-native-support`) is unaffected by this work and should stay open independently.
