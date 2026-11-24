# gamepad-wheel

Turns an Android phone (gyroscope) + a GameSir Nova 2 Lite into a single virtual Xbox 360 controller on Windows: the phone, clipped to the gamepad, steers; the gamepad does everything else. PC side in Rust (ViGEmBus + HidHide), phone app in native Kotlin, UDP over Wi-Fi. Open source (MIT OR Apache-2.0).

- Product source of truth: `docs/PRD.md` (Portuguese). Current phase: **phase 0 spikes** (Q1–Q7 decide how R3, R8, R11 are built).
- Layout and toolchains: ADR 0001 (`desktop/` Rust, `mobile/` Android, `spikes/`). Run all checks with `scripts/check.sh` (Rust runs in Docker). Do not add a framework without an issue/decision on record.
- Docs, code comments, commits, issues and PRs are written in **English**.

## Windows testing (WinLab VM)

The dev machine is NixOS; Windows-only code runs in the VirtualBox VM **WinLab** (Windows 11, ViGEmBus, HidHide, Rust, VS Build Tools; no games).

- Drive it from the host: `VBoxManage controlvm WinLab screenshotpng <file>` and `keyboardputstring`. Once `W:\kit\06-claude-runner.ps1` is running in an admin PowerShell, drop scripts into `/mnt/win/VMShare/pessoal/jobs/` (`W:\jobs`) and read `W:\out\<name>.txt` (ends with `__EXIT <code>`).
- The repo is shared read-only as `\\VBoxSvr\gamepad-wheel`; build under `C:\src` (`CARGO_TARGET_DIR=C:\build\target`).
- Never take VirtualBox snapshots: the disk is shared with the dual-boot Windows.
- The Nova 2 Lite cannot stay in XBOX mode inside the VM (the dongle falls back to DS4 mode when a Linux host enumerates it first). Real-controller checks run on the dual-boot Windows; in the VM a ViGEm pad stands in for it (`spikes/winlab-check.ps1`).

## Branches (gitflow)

`develop` is the default branch. Always branch `feature/*` or `fix/*` from it; `main` only receives `release/*` and `hotfix/*`. Full rules: `.claude/rules/gitflow.md` and `.claude/rules/commits-and-prs.md`.

## Skills (mattpocock-skills plugin)

- First time in the repo: `/setup-matt-pocock-skills` (tracker = GitHub Issues).
- Before changing anything: `/grill-with-docs`.
- PRD → issues: `/to-spec`, `/to-tickets`. Spikes Q1–Q7: `/prototype`.
- Implement: `/tdd`. Bugs: `/diagnosing-bugs`. Open a PR: `/pr`.

## MCPs (`.mcp.json`)

- **maestro**: drive the Android app on a device/emulator (R1, R2, R12, R13 flows). Requires the Maestro CLI on `PATH` and Java 17+.
- **playwright**: only useful if Q7 picks Tauri (web GUI). Unused with egui.
- **context7**: current docs for crates/SDKs (`vigem-rust`, egui, Jetpack Compose, Android sensors). Use before writing library-dependent code.

## Agent skills

### Issue tracker

Issues live in GitHub Issues (`gh` CLI). See `docs/agents/issue-tracker.md`.

### Triage labels

Default vocabulary: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `GLOSSARY.md` + `docs/adr/` at the repo root. See `docs/agents/domain.md`.
