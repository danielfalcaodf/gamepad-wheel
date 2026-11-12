# Gitflow

| Branch | From | PR into | Merge |
|---|---|---|---|
| `feature/<issue>-<slug>` | develop | develop | squash |
| `fix/<issue>-<slug>` | develop | develop | squash |
| `release/x.y.z` | develop | main **and** develop | merge commit |
| `hotfix/<slug>` | main | main **and** develop | merge commit |

- `main` = production; `develop` = integration and the repo's **default branch**.
- Never commit or push directly to `main` or `develop`; everything goes through a PR.
- After merging `release/*` or `hotfix/*` into `main`, tag `vX.Y.Z` on `main`.
- Delete the remote branch after merge.

## Recommended protection (main and develop)

- PR required; no direct push; no force push; no deletion.
- Conversations resolved before merge.
- Required status checks: **none yet** (no CI). Mark the first workflow as required once it exists.
- Approvals: 0 while solo; raise to 1 when outside contributors arrive (CODEOWNERS review).
