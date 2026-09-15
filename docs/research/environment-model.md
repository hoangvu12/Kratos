# The t3code/laplus environment model — research

How t3code (pingdotgg/t3code) and our laplus (`../laplus-next`, a t3code fork) connect clients to environments **without any account or cloud**, and what that would mean for roboco. Companion to `t3code-auth.md` (which covers the optional Clerk-gated relay).

Sources: laplus source at `../laplus-next/server/crates/laplus-server/src/` (pairing.rs, auth.rs, codes.rs, remote_access.rs, `server/docs/running-headless.md`); upstream t3code at github.com/pingdotgg/t3code (cited from laplus doc comments).

## The model

**The environment is the unit, and it is self-sovereign.** Each machine runs one server (t3code "environment" / laplus-server). It owns its own sessions, its own SQLite identity store, its own credentials. There is no account anywhere, no cloud dependency, no login.

**Default posture: loopback.** The server binds 127.0.0.1; the desktop window talking to its own local server is the whole product. Remote access is opt-in: a remote-access file next to settings.json, or `--network lan` / env override for headless boxes (`remote_access.rs`). Exposure to the internet is tunnels (cloudflared) or LAN — the server itself never needs a public port.

**The credential chain** (`pairing.rs` — "A phone reaching this machine through a tunnel walks four steps"):

1. **Pair code** — 12 chars from a 32-char ambiguity-free alphabet (~60 bits), 5-minute TTL, single use, read off one screen and typed into another. Minted from Settings, or `auth pairing create` from a terminal (the row in SQLite is the code's whole existence — a second process can mint one the running server honors). QR encodes the full pairing URL including the credential.
2. `POST /oauth/token` — trades the code for a **session token**, 30-day bearer. This is what the paired client keeps.
3. `POST /api/auth/websocket-ticket` — trades the bearer for a **ws ticket**, 5 minutes, one upgrade. Exists because browsers can't set WS headers; a 30-day credential must not ride in a query string and end up in logs.
4. `GET /ws?wsTicket=…` — the socket.

The desktop window uses a **boot grant** (24h, survives reloads, short enough that a leaked one dies on its own). All randomness from `getrandom` (OS CSPRNG), not hash seeds or SQLite randomblob.

**Scopes** are carried on pairing codes and enforced on public-exposure surfaces (`docs/adr/0047` in laplus): a paired phone has no business administering tunnels.

**Multi-environment clients.** A client (phone, desktop, web) can hold several paired environments — your PC and your linux server — and switch between them. Pairing is just "open the link / type the code"; the client stores a base URL + session token per environment.

**Where identity-as-a-service appears at all:** only in the *optional* managed relay (t3code's T3 Connect; Clerk). Self-hosting that is out of scope of the core model — plain tunnels cover the "phone → home box" case without it.

## Contrast with zeron/roboco's current sync

| | zeron (current roboco) | t3code/laplus |
|---|---|---|
| Unit of identity | Account (WorkOS) + org | The environment itself |
| Hub | Cloud edge (CF Worker + DO rooms) | None — direct connections |
| Session data | loro CRDT docs synced *through* the edge | Server-owned; clients are views/control over RPC |
| Client attachment | Sign in; devices discover each other via account | Pair with a link; N environments per client |
| Remote access | Always via edge relay | LAN / tunnel, opt-in |
| Works offline | Local-only mode | That's the only mode |

## What this would mean for roboco

Roboco already has the hard parts as *local* machinery: a per-device engine, a device-room RPC byte pipe (currently host-dials-out to the edge DO and clients multiplex through it), local loro docs. The laplus-shaped change:

1. **Engine listens** instead of dialing out: HTTP+WSS server on loopback by default, opt-in LAN/remote (laplus `remote_access.rs` posture).
2. **Pairing, not login**: port the credential chain above (pair code → session → ws ticket; boot grant for the bundled desktop window). Delete the WorkOS state machine in `crates/engine/src/auth.rs`, the edge's `/auth/*` routes, org onboarding.
3. **Multi-env shell**: the desktop app becomes a client that can attach to N engines (local + linux server), with an environment picker. Remote engine = the same RPC surface, pointed at a paired URL (LAN or cloudflared tunnel).
4. **Delete**: `edge/` (Worker + SessionRoom/RegistryRoom/ChatRoom/PreviewRoom), WorkOS, `roboco-sync` room clients. Local loro docs stay — they're the engine's storage either way.
5. **Cost**: this is the structural divergence I warned about — session sync between *your own devices* (CRDT rooms) disappears in exchange for direct client→engine control. Upstream merges in engine/shell get harder; the sync crates effectively fork.

Laplus implementation notes worth stealing verbatim: the pairing-code alphabet/TTL choices, ticket-for-query-string discipline, "the row is the code's whole existence" (second process can mint), loopback-is-the-posture defaults, and scopes-only-on-public-exposure.
