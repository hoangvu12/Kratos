# 09 — Per-engine routing

**What to build:** Requests go to the right socket: every chat, space, file, and terminal operation routes through its own engine's connection. The space palette's device tabs enumerate paired engines; the composer's space chip shows engine+folder for the next chat; creating a chat from a remote engine's space runs it there end-to-end.

**Blocked by:** 08 — Client engine registry + merged sidebar.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] New chat created in a remote engine's space runs on that engine (transcript, queue, files all remote)
- [ ] Terminal and file views of a remote chat operate through the remote connection
- [ ] Composer chip and palette reflect the true target before sending
- [ ] A request aimed at a disconnected engine fails fast with the compact offline indication — never silently misroutes to another engine
