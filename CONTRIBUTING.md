# Contributing

1. Open or pick an issue.
2. `git switch develop && git pull`
3. `git switch -c feature/<issue>-<slug>` (or `fix/...`).
4. Commit with [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
5. Open a PR into `develop` using the template. PRs are **squash**-merged.

Releases: `release/x.y.z` from `develop` → PR into `main` (merge commit) → tag `vX.Y.Z` → PR back into `develop`.
Hotfixes: `hotfix/<slug>` from `main` → PRs into `main` and `develop` (merge commit).

Full rules: [.claude/rules/gitflow.md](.claude/rules/gitflow.md) and [.claude/rules/commits-and-prs.md](.claude/rules/commits-and-prs.md).

By contributing you agree your work is dual licensed under MIT OR Apache-2.0 (see [README](README.md#license)).
