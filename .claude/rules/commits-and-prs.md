# Commits and PRs

## Commits: Conventional Commits 1.0

`<type>(<optional scope>): <imperative, lowercase description, no period>`

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`, `build`, `perf`, `style`, `revert`.
Suggested scopes (from the PRD): `pc`, `android`, `protocol`, `mixer`, `vigem`, `hidhide`, `gui`, `profiles`, `docs`.
Breaking change: `!` after the type or a `BREAKING CHANGE:` footer. Any change to the UDP wire format is breaking and bumps the protocol version.
Reference the issue in the footer: `Refs #12` / `Closes #12`.

## PRs

- PR title = squash commit message (Conventional Commits).
- Base: `develop` (feature/fix) or `main` (release/hotfix).
- Fill in the template: what changes, issue, how to test, checklist.
- One topic per PR; keep PRs small.
