# 02 — Delete edge worker + iOS app + dead workflows

**What to build:** The cloud that no longer exists stops being in the repo: the edge worker (all Durable Object rooms, auth routes, install surface), the iOS app, and the workflows/deploy pieces that only served them are deleted. The workspace and CI build green without them.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [x] Edge worker directory, iOS app directory, and their workflows are gone
- [x] No Rust/TS reference to the deleted trees remains in build or CI config
- [ ] `cargo check --workspace` and the remaining CI workflows pass

## Implementation

Removed the edge worker, iOS app, upstream landing/redirect deployments, and their
smoke/compatibility fixtures. Removed the daemon test's compile-time dependency on
the deleted installer. Deployment and TestFlight workflows were already absent.

Validation: `cargo check --workspace --locked`, `cargo check --locked -p roboco --tests`,
and `git diff --check` pass on Windows. Build/CI/script configuration contains no
references to the deleted directories. Hosted Windows/Linux CI remains to be run.
