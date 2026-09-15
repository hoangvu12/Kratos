# Roboco — fork workflow

Roboco (repo/project/binary: `roboco`, app display name: `Roboco`) is a hard fork of [zeronsh/zeron](https://github.com/zeronsh/zeron) with native Windows support. `main` = upstream zeron `main` + the [Windows native support PR line](https://github.com/zeronsh/zeron/pull/313). The wasimysaid Kratos line (Mimir ACP etc.) is intentionally **not** merged here; it lives in `../Kratos`.

## Remotes

- `origin` → `hoangvu12/roboco` — our direct fork of `zeronsh/zeron` (holds the open zeron PRs)
- `upstream` → `zeronsh/zeron`
- `kratos` → local `../Kratos` checkout (reference only)
- `rerere` is enabled — keep it that way; rebrand conflicts repeat and get auto-resolved.

## Pulling from zeron

```bash
git fetch upstream
git merge upstream/main
```

Expect conflicts wherever upstream touches what we renamed. Resolution rule: take upstream's content, then re-apply the rename mapping:

- `zeron-*` crates / `zeron_*` libs → `roboco-*` / `roboco_*`
- `apps/zeron/` → `apps/roboco/`
- `ZERON_*` env vars → `ROBOCO_*`
- `sh.zeron.*` bundle ids → `sh.roboco.*`, `zeron://` links → `roboco://`

## PRing to zeron

Never branch off roboco `main` for zeron PRs — it carries the rebrand. Branch off upstream:

```bash
git checkout -b fix/whatever upstream/main
# ... hack ...
git push origin fix/whatever
gh pr create -R zeronsh/zeron --base main --head hoangvu12:fix/whatever
```

`windows-native-support` on `origin` is the head of open PR #313 — do not delete or rebase casually.

## GPUI forks

GPUI comes from our forks, pinned by rev in the root `Cargo.toml`:

- `hoangvu12/zui` (rev `aa009411…`) — currently identical to `zeronsh/zui` main
- `hoangvu12/gpui-component` (rev `94c1bbaf…`)

`gpui-component` still declares its gpui crates against `zeronsh/zui`, so the `[patch."https://github.com/zeronsh/zui"]` section redirects them to `hoangvu12/zui`. **Rule: the patch rev must always equal the top-level `gpui` pin rev**, otherwise you get two GPUI copies and ~50 type-mismatch errors. To bump: push/verify the rev exists in `hoangvu12/zui`, then update the pins and the patch revs together.

Custom gpui work goes on branches of `hoangvu12/zui` first, then gets pinned here by rev.

## Rebrand boundaries (do not "fix" these)

Still zeron-branded on purpose:

- `zeronsh` org references and PR/issue links
- `zeron.sh` / `edge.zeron.sh` URLs — the app syncs via zeron's public edge; we don't run our own
- `apps/ios/`, `apps/landing/`, `apps/www-redirect/`, `edge/` — upstream's deployable infra
- `docs/research/` — historical research notes
- `ZERON_GPU_STATS` — env var owned by the zui fork, not this repo

## CI (Windows + Linux only)

- `windows.yml` — Windows tests (PR + push)
- `ui-tests.yml` — ubuntu jobs only (session sync, UI regressions, linux browser)
- `preview-tests.yml` — ubuntu (preview/proto tests)
- `release.yml` — tag `v*`: linux x86_64+aarch64 tarballs + windows portable zip → GitHub Release with `manifest.json` (updater checksums). No macOS/iOS/R2.
- Deleted on purpose: `deploy.yml` (zeron.sh infra), `testflight.yml` (iOS). Expect these to reappear on upstream merges — delete them again in the merge commit.

## Naming conventions

Like zeron/Zeron: lowercase `roboco` for repo, crates, binary, package names, env prefix (`ROBOCO_`), deep link (`roboco://`); capitalized `Roboco` for app display name, window titles, UI strings, prose.

## Windows development

See `docs/reference/windows-development.md`. Env vars use the `ROBOCO_` prefix (e.g. `ROBOCO_DATA_DIR`).
