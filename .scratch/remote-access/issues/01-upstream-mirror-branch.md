# 01 — Upstream mirror branch + cherry-pick policy

**What to build:** A pristine, un-renamed Zeron mirror branch lives in this repo (`zeron/main`, tracking upstream `zeronsh/zeron` main, never merged into our main), and AGENTS.md documents the port workflow: cherry-pick upstream commits onto the mirror, then carry the change across the rebrand and cutover by hand. This is the standing mechanism ADR 0003 requires.

**Blocked by:** None — can start immediately.

**Status:** resolved

**Parent:** `.scratch/remote-access/spec.md`

- [x] `zeron/main` branch exists in-repo, content-identical to upstream main (un-renamed crate names, paths)
- [x] AGENTS.md upstream section rewritten: no merges, port workflow documented
- [x] A dry-run port of one trivial upstream commit proves the workflow end-to-end

## Implementation evidence ? 2026-09-15

- Local `zeron/main` tracks `upstream/main`; both resolve to
  `6d39f8c6a8f3c62a332f2abbafe989177da289e9` after fetching upstream.
  `git diff --exit-code zeron/main upstream/main` passed. Branch refs/config
  are repository-local state and are not transported by cherry-picking this ticket.
- AGENTS.md now points to `docs/reference/upstream-ports.md` for mirror refresh,
  temporary upstream replay, manual Roboco adaptation, validation and cleanup.
- Replayed actual upstream `dfcf29354f520bc47fab8e2fcd53c37cc9bbe987`
  (Fedora browser dependency docs and build diagnostic) from its parent in
  `proof/remote-01-upstream`, producing `49045147`. Full-tree equality with the
  original upstream commit passed.
- Since that historical change is already present in Roboco, prepared a
  pre-change baseline (`864f4558`) only in `proof/remote-01-roboco`, branched
  from `main` at `c91d9b39`. Cherry-picked the upstream replay with `-x` and
  manually resolved the documentation conflict, preserving Roboco branding.
  Result: `9f83c32363d2ca69bf9b3436d32ba5128a92fdf6`.
- Validation: build.rs exactly matches the selected upstream version; the
  documentation exactly matches upstream after `Zeron` ? `Roboco`; the replay
  changes exactly those two files; whitespace checks and clean-worktree checks
  passed. This documentation/diagnostic replay did not run Linux compilation
  on the Windows host. It proves port mechanics, not Linux runtime behavior.
- Main stayed at `c91d9b39bc39b9619ec9ef4d7f2a14bd25c032d4` throughout the
  proof. Neither proof branch was merged. Temporary worktrees/branches were
  removed after validation; hashes above are evidence, not durable refs.
