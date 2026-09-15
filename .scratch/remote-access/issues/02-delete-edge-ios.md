# 02 — Delete edge worker + iOS app + dead workflows

**What to build:** The cloud that no longer exists stops being in the repo: the edge worker (all Durable Object rooms, auth routes, install surface), the iOS app, and the workflows/deploy pieces that only served them are deleted. The workspace and CI build green without them.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] Edge worker directory, iOS app directory, and their workflows are gone
- [ ] No Rust/TS reference to the deleted trees remains in build or CI config
- [ ] `cargo check --workspace` and the remaining CI workflows pass
