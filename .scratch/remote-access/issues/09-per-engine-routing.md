# 09 — Per-engine routing

**What to build:** Requests go to the right socket: every chat, space, file, and terminal operation routes through its own engine's connection. The space palette's device tabs enumerate paired engines; the composer's space chip shows engine+folder for the next chat; creating a chat from a remote engine's space runs it there end-to-end.

**Blocked by:** 08 — Client engine registry + merged sidebar.

**Status:** implemented; integration verification in progress

**Parent:** `.scratch/remote-access/spec.md`

- [ ] New chat created in a remote engine's space runs on that engine (transcript, queue, files all remote)
- [ ] Terminal and file views of a remote chat operate through the remote connection
- [ ] Composer chip and palette reflect the true target before sending
- [ ] A request aimed at a disconnected engine fails fast with the compact offline indication — never silently misroutes to another engine

## Verification

`cargo test -p roboco-ui --lib request_routing::tests -- --test-threads=1`
passed all three tests on Windows (2026-09-15). The real-engine test starts two
independent engines with identical chat and space IDs, pairs the second engine,
and checks separate file contents, a remote file write that leaves the local
file unchanged, colliding upload IDs with distinct PNG readbacks, transcript
subscription, and terminal open/resize/write/close. After the remote listener
stops, its captured target fails within 250 ms while the local file remains
readable. Terminal IDs are server-minted; this test exercises actual terminal
ownership rather than forcing an unsupported duplicate terminal ID.

Pure boundary tests check that request identity fields are decoded without
rewriting user text, paths, or arbitrary options, and reject foreign scoped
identities (including explicitly scoped local IDs on a remote connection).

Consumers now retain an engine target across asynchronous uploads and requests.
File request context and attachment cache keys retain engine provenance.
The composer checks connection state before creating an optimistic message.
Full integrated UI validation and transcript frame projection are tracked with
tickets 08 and 10. Remote browser previews currently require a tunnel URL;
engine-local localhost URLs are not opened on the client.
