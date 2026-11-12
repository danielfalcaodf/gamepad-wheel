# gamepad-wheel

Turns an Android phone (gyroscope) + a GameSir Nova 2 Lite into a single virtual Xbox 360 controller on Windows: the phone, clipped to the gamepad, steers; the gamepad does everything else. PC side in Rust (ViGEmBus + HidHide), phone app in native Kotlin, UDP over Wi-Fi. Open source (MIT OR Apache-2.0).

- Product source of truth: `docs/PRD.md` (Portuguese). Current phase: **phase 0 spikes** (Q1–Q7 decide how R3, R8, R11 are built).
- No stack is installed yet. Do not scaffold a framework without an issue/decision on record.
- Docs, code comments, commits, issues and PRs are written in **English**.

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
- **context7**: current docs for crates/SDKs (`vigem-client`, egui, Jetpack Compose, Android sensors). Use before writing library-dependent code.
