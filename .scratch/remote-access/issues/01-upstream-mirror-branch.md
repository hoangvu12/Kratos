# 01 — Upstream mirror branch + cherry-pick policy

**What to build:** A pristine, un-renamed Zeron mirror branch lives in this repo (`zeron/main`, tracking upstream `zeronsh/zeron` main, never merged into our main), and AGENTS.md documents the port workflow: cherry-pick upstream commits onto the mirror, then carry the change across the rebrand and cutover by hand. This is the standing mechanism ADR 0003 requires.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] `zeron/main` branch exists in-repo, content-identical to upstream main (un-renamed crate names, paths)
- [ ] AGENTS.md upstream section rewritten: no merges, port workflow documented
- [ ] A dry-run port of one trivial upstream commit proves the workflow end-to-end
