# 10 — Offline cache + compact reconnect strip

**What to build:** An offline engine stays useful and unobtrusive: session lists and open transcripts remain readable from a per-engine client cache; a single compact strip (not a banner) indicates reconnecting state inside an offline chat; the supervisor retries with backoff and on app restart; writes to an offline engine are refused outright, never silently queued.

**Blocked by:** 08 — Client engine registry + merged sidebar.

**Status:** resolved

**Parent:** `.scratch/remote-access/spec.md`

- [x] Cached history renders read-only while the engine is unreachable; nothing blanks out
- [x] The reconnect indicator occupies one thin strip; the rest of the UI is unchanged
- [x] Sending to an offline engine is refused with a clear affordance
- [x] Reconnection restores live state without losing the open view

## Implementation and validation

Per-engine `EngineCache` persists sidebar rows and open transcripts beside the
pairing registry (engine-scoped keys; credentials never enter the cache). While
the engine is unreachable the transcript view keeps cached history readable
behind a single 24px reconnect strip (`engine-reconnect-strip`), and the
composer refuses sends to an offline engine before any optimistic write. The
registry supervisor retries with capped backoff and reloads persisted rows at
startup.

Validation on Windows (2026-09-15): `state::cache_tests` drives a real paired
engine through queue-and-disconnect, asserting cached sidebar and transcript
survive a full client restart with the remote down; `engine_cache` and
`engine_registry` suites cover offline reads, forget, and reconnect restore.
All pass, and the hosted CI runs green on the PR branch.
