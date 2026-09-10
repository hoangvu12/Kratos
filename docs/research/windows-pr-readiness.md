# Windows PR readiness research

## Accepted delivery plan

The contributor chose one Comet Windows-support PR after reviewing the initial
research. The multi-PR sequence below is retained as an alternative considered
during research, not the current plan. Organize native runtime changes, the
temporary renderer patch, and CI/documentation into coherent commits in one PR.
Keep fixes required by Windows together. Upstreaming the renderer to zui is a
separate follow-up and does not block this cleanup.

The cleanup adds the missing Codex integration CI targets, shared Linux/macOS
regression jobs, and a concise development guide. Historical machine-specific
evidence is preserved in [the verification archive](windows-verification-history.md).

Research date: 2026-09-11. Local base: `c31b440`, workspace version `0.2.54`.
This report examines the existing working-tree changes; it does not implement a
cleanup. Upstream freshness was not checked, so the PR sequence below must be
reconciled with the target branches before implementation. No build, tests, or
GitHub Actions jobs were run for this report. Historical test results cited below
are claims in the existing development notes, not fresh verification.

## Main recommendation

Keep the established crate boundaries and split the Windows work by behavior and
dependency owner. Submit the DirectX renderer repair to `zeronsh/zui`; keep app
paths, engine ownership, agent launching, terminal lifecycle, and application
integration in Comet. A broad new platform crate or repository-wide reformat is
not needed to make this work reviewable.

This follows the existing UI/engine/RPC/harness organization in
[ARCHITECTURE.md](../../ARCHITECTURE.md), the shared pinned GPUI dependencies in
[Cargo.toml](../../Cargo.toml), and the explicitly scoped backend patch described
in [vendor provenance](../../vendor/gpui_windows/UPSTREAM.md). It is a recommendation,
not an assertion that maintainers have approved this split.

## Observed state

- `git status --short` showed 31 modified tracked files, plus new tests, fixtures,
  PowerShell probes, Windows documentation, CI, and the vendor directory. Ordinary
  `git diff --stat` excludes those untracked additions, so it understates the
  review surface. The 27-file vendor copy is especially important to count.
- [Cargo.toml](../../Cargo.toml) pins GPUI-family dependencies to zui
  `07fd941ad72e7edc812fed317aab66adb69fa8cc` and patches only `gpui_windows` to a
  local path. [Its standalone manifest](../../vendor/gpui_windows/Cargo.toml)
  repeats that pin for related crates. [UPSTREAM.md](../../vendor/gpui_windows/UPSTREAM.md)
  identifies the original revision, retained Apache license, shader layout/fade
  changes, and focused tests. It explicitly excludes backdrop blur support.
- [Existing macOS CI](../../.github/workflows/ui-tests.yml) already checks out
  the zui revision from Comet's manifest and runs native backend tests there.
  This is a useful local precedent for dependency-owned regression coverage.
- [Windows CI](../../.github/workflows/windows.yml) includes release linking,
  shader layout tests, application/updater tests, harness and engine library
  tests, native ACP fixture tests, UI tests, and startup without `HOME`.
  Rendering and lifecycle probes run only through manual `native_gui` dispatch.
- The workflow does not invoke the new
  [Codex availability integration test](../../crates/harness/tests/codex_availability.rs)
  or [production catalog integration test](../../crates/engine/tests/codex_catalog.rs).
  The relevant steps use `--lib`, which does not select these integration targets.
  These tests inspect synthetic native payloads and discovery/catalog behavior;
  they do not prove an authenticated provider session works.
- [Windows development notes](../reference/windows-development.md) explicitly
  say the workflow has not run on GitHub Actions. They distinguish historical
  local checks from unverified agent process-tree, GPU/DPI, release, and parity
  behavior. [Release CI](../../.github/workflows/release.yml) still packages Linux
  and macOS, and publication depends on those two jobs.
- The development guide mixes reusable instructions with machine-specific NVM
  paths, a sibling `laplus-next` checkout, an old unsuccessful workaround, a
  dated verification table, and continuation history. The repository already
  separates [reference guides](../reference/windows-development.md) from
  [research notes](windows-support.md), so this can be tidied without inventing
  a new documentation hierarchy.
- No `AGENTS.md` was found in the repository or the checked immediate ancestor
  directories. This report therefore does not infer undocumented contributor
  rules or a required formatting/lint policy.

## Proposed PR sequence

Each row is a proposed review unit, not a change made by this research. PRs should
carry their own relevant tests, documentation, and runnable CI steps. Do not land
the complete current workflow ahead of the fixtures and features it invokes.

| Order / owner | Reviewable behavior | Scope and dependency |
| --- | --- | --- |
| A / zui, parallel with Comet foundations | Correct Windows scene/shader ABI and existing edge fades | Port the actual backend delta and layout tests from `vendor/gpui_windows`; compare against the exact recorded source revision. Keep standalone vendoring manifest mechanics out of the upstream patch. Include before/after synthetic pixels and explicit unsupported blur behavior. |
| 1 / Comet | Native Windows data directories and single engine ownership | Application paths and their callers, Windows file locking, daemon/updater platform behavior, and directly necessary portable fixtures. Establish the minimum Windows build/startup/test job that this branch can pass. Split updater guards into a separate small PR if independently buildable. |
| 2 / Comet | Discover and launch native agents consistently | Shared executable resolution, managed adapter launch planning, harness call sites, native fixture and availability/catalog coverage. Preserve argument arrays and explicit batch rejection. Include tests demonstrating that an available native npm payload reaches the production harness catalog. |
| 3 / Comet | Windows terminal lifecycle | Engine PTY/process ownership and the corresponding UI integration, with focused ConPTY lifecycle regressions. Depends on PR 2 wherever it consumes executable discovery. Keep documented direct-child behavior distinct from future process-tree termination. |
| 4 / Comet | Consume the repaired renderer | After A lands, update related zui pins consistently and regenerate the lockfile; remove the local override/vendor copy as part of that dependency transition. Keep application rendering fixture, bounded capture helper, synthetic screenshots, and native probe instructions here. |
| 5 / Comet, independently where possible | Shared behavior fixes discovered during Windows validation | Separate bootstrap IPC cancellation cleanup and remote path classification from platform launch work when their tests can run independently. Retain the UI/engine ownership boundary and exercise Windows views of Unix-hosted workspaces. |

The proposed scopes come from the changed
[application paths](../../apps/zeron/src/paths.rs),
[instance lock](../../crates/engine/src/instance_lock.rs),
[updater](../../crates/update/src/lib.rs),
[executable resolver](../../crates/harness/src/executable.rs),
[adapter installer](../../crates/harness/src/adapter_install.rs),
[terminal owner](../../crates/engine/src/terminals.rs),
[UI bootstrap](../../crates/ui/src/state.rs), and
[workspace links](../../crates/ui/src/workspace_links.rs).
Actual hunk dependencies need checking when preparing branches; file ownership
alone is insufficient for splitting shared manifest edits and overlapping tests.

If zui integration cannot land promptly, a temporary Comet renderer PR remains
possible: retain the license and provenance, link the proposed upstream change,
record exactly which files differ, and state removal conditions. Do not silently
turn this scoped patch into a second independently maintained GPUI stack.

## Code structure findings

Preserve the new [executable resolver](../../crates/harness/src/executable.rs)
as the owner of native discovery. Moving shared discovery out of ACP and the
harness root already reduces callers' knowledge of platform details. The existing
[browser module](../../crates/ui/src/browser/mod.rs) demonstrates another local
pattern: platform-specific implementations sit behind a shared module interface.
Use that pattern where complexity warrants it; do not create a generic platform
crate merely to collect every Windows condition.

[Application paths](../../apps/zeron/src/paths.rs) and
[managed adapter roots](../../crates/harness/src/adapter_install.rs) overlap in
their Windows application-data policy, while provider credential/home locations
have a different purpose. Document those policies first; a later resolved-root
parameter may remove duplicated decisions without conflating provider homes and
Zeron storage. The existing acquire/holder interface in
[instance_lock.rs](../../crates/engine/src/instance_lock.rs) is already a useful
boundary. Duplicated already-running error construction is minor cleanup, not a
reason to redesign engine ownership.

The changes to [terminal output pumping](../../crates/engine/src/terminals.rs)
include concurrent child waiting/reading, dropping blocking handles, and
`Arc`/`Weak` ownership. Those paths affect Unix too, strengthening the case for
the separate lifecycle PR and Linux/macOS evidence. Likewise,
[UI bootstrap's abort-and-await IPC cleanup](../../crates/ui/src/state.rs) is a
general lifecycle fix suitable for an independent regression PR.

The diffs in [source-control code](../../crates/engine/src/source_control.rs)
concern test portability rather than changed production source-control behavior.
The changes in [document schema](../../crates/doc/src/schema.rs),
[Claude normalization](../../crates/harness/src/claude/normalize.rs), and
[Codex normalization](../../crates/harness/src/codex/normalize.rs) are formatting
only in the reviewed working tree. Exclude that formatting from future Windows
PRs; no existing edits were removed by this research.

## Validation to make the PRs reviewable

These are proposed checks, not results from this research:

1. Make the existing Windows build and regression workflow run on the proposed
   branches; attach its first actual result. Preserve release linking and shader
   compilation, since `cargo check` does not establish renderer readiness.
2. Add explicit Windows invocations for
   `cargo test --release --locked -p zeron-harness --test codex_availability` and
   `cargo test --release --locked -p zeron-engine --test codex_catalog` to the
   relevant PR. Keep the existing `native-fixture` invocation with the native
   launch changes. Consider other integration targets individually instead of
   assuming `--lib` covers them.
3. Run relevant harness/engine regressions on Linux and macOS when shared launch,
   path, or normalization code changes. Existing
   [UI CI](../../.github/workflows/ui-tests.yml) exercises UI and native surfaces,
   but is not a substitute for those changed crate suites.
4. Continue treating native rendering/lifecycle probes as separately reported
   evidence. Their current manual dispatch gate must not be described as PR-time
   GPU coverage. The [rendering script](../../scripts/test-windows-rendering.ps1)
   calls [its bounded capture helper](../../scripts/test-windows-capture-client.ps1);
   keep both in the same review unit.
5. When moving shader tests upstream, adapt CI to test the actual consumed zui
   revision, following the macOS precedent. Explain what CPU/HLSL source layout
   tests prove and what native pixels add. Multi-DPI and additional GPU coverage
   remain separate from the recorded single-machine 96-DPI evidence.

## Documentation and review preparation

Make [windows-development.md](../reference/windows-development.md) a concise
current guide: prerequisites, debug launch, focused test commands, configuration,
and known limitations. Move local paths, sibling-checkout comparisons, failed
workarounds and dated observations into a clearly dated research/evidence note.
Retain their factual uncertainty rather than converting past results into claims
about an arbitrary future commit. Keep the existing historical
[windows-support.md](windows-support.md) explicitly historical.

For every PR, explain one concrete trigger and resulting behavior, the platform
scope, dependency links, executed validation with host/revision, and outstanding
limitations. For example: "Windows with npm Codex on PATH previously omitted the
provider from the catalog; discover its native payload and use the same resolver
for availability and launch." Include the catalog test, not just a file list.

Prepare branches only after preserving the complete existing work, including
untracked files. Compare against the chosen current upstream base before moving
commits or hunks; upstream freshness is outside this report. Keep line-ending or
format-only churn separate if needed, rather than sweeping unrelated files into
the behavioral changes. Do not advertise Windows release support until the
separate packaging/update acceptance work is actually complete.
