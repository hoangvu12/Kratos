# 11 — Add-space manual path + create-if-missing

**What to build:** In the add-space palette, a typed path is as good as a browsed one: Enter submits the manual path against the chosen engine, and if the folder doesn't exist the engine creates it remotely (button flips to a create-and-add affordance). Path validity is judged by the target engine's platform, not the client's.

**Blocked by:** 09 — Per-engine routing.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] Typing a path to an existing folder on a remote engine adds the space
- [ ] A missing path offers create-and-add; the engine mkdirs and the space lands
- [ ] Windows-style paths are rejected on non-Windows engines and vice versa
- [ ] Hidden dot-folders appear only when the query starts with `.`
