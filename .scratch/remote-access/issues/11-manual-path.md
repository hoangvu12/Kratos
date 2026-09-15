# 11 — Add-space manual path + create-if-missing

**What to build:** In the add-space palette, a typed path is as good as a browsed one: Enter submits the manual path against the chosen engine, and if the folder doesn't exist the engine creates it remotely (button flips to a create-and-add affordance). Path validity is judged by the target engine's platform, not the client's.

**Blocked by:** 09 — Per-engine routing.

**Status:** resolved

**Parent:** `.scratch/remote-access/spec.md`

- [x] Typing a path to an existing folder on a remote engine adds the space
- [x] A missing path offers create-and-add; the engine mkdirs and the space lands
- [x] Windows-style paths are rejected on non-Windows engines and vice versa
- [x] Hidden dot-folders appear only when the query starts with `.`

## Implementation and validation

The add-space palette probes a path-shaped query against the selected engine
(`PREPARE_SPACE_PATH`), flipping the submit affordance to create-and-add when
the folder is missing; the engine interprets validity on its own platform and
mkdirs on demand.

Validation (2026-09-15): the engine test
`remote_manual_paths_are_checked_and_created_on_the_engine` drives a real
authenticated listener through probe, create, add, dot-folder gating, and
foreign-platform rejection, and the UI test
`manual_remote_path_enter_and_create_use_selected_engine` exercises the palette
interaction end to end. The palette's edit subscription originally missed the
probe (the handler sat on the spaces menu's search); fixed on the PR branch.
Both pass, and the hosted CI runs green.
