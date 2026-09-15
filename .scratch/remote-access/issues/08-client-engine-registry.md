# 08 — Client engine registry + merged sidebar

**What to build:** The shell stops assuming one engine connection. A registry holds the local engine plus every paired remote (pairing URL paste-in for the first version), each with its own supervised connection that auto-connects at startup and reconnects with capped backoff. Sessions from all engines appear in the existing sidebar — same rows, same `@ device` tags, same device grouping — with connection-status dots on engine surfaces. No new sidebar components.

**Blocked by:** 06 — Authenticated remote listener.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] Adding a remote engine by URL connects it; its sessions appear beside local ones under its device label
- [ ] Removing/forgetting an engine drops only its rows
- [ ] Killing a remote engine marks it reconnecting; recovery restores its rows without app restart
- [ ] Registry behavior verified by tests against real engines on loopback ports (spec's seam)
