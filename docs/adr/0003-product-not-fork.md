# Roboco is a product, not a tracking fork

Roboco started as a fork of zeronsh/zeron (rebranded), but we remove zeron's cloud sync (edge Worker, WorkOS auth, sync rooms) and replace remote access with direct engine pairing. Upstream is therefore no longer merged; interesting zeron features are cherry-picked or ported when we want them. Keeping a pristine `zeron/main` mirror branch in-repo makes those ports tractable despite the rebrand and removals.

## Considered Options

- **Tracking fork** (merge upstream regularly, keep sync dormant) — rejected: the account/cloud model conflicts with the pairing model, and merge-conflict tax keeps being paid for code we deleted on purpose.
- **Product, cherry-pick only** (chosen) — accepted one-way cost: ports become manual work; in exchange the codebase stays ours.

## Consequences

- "Pulling from zeron" changes from `git merge upstream/main` to porting selected commits (via the pristine mirror branch).
- Deletion of sync/edge/WorkOS happens as a deliberate cutover, not a deferred cleanup.
