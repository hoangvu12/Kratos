# 10 — Offline cache + compact reconnect strip

**What to build:** An offline engine stays useful and unobtrusive: session lists and open transcripts remain readable from a per-engine client cache; a single compact strip (not a banner) indicates reconnecting state inside an offline chat; the supervisor retries with backoff and on app restart; writes to an offline engine are refused outright, never silently queued.

**Blocked by:** 08 — Client engine registry + merged sidebar.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] Cached history renders read-only while the engine is unreachable; nothing blanks out
- [ ] The reconnect indicator occupies one thin strip; the rest of the UI is unchanged
- [ ] Sending to an offline engine is refused with a clear affordance
- [ ] Reconnection restores live state without losing the open view
