---
status: accepted
---

# Native Cargo + Gradle monorepo, no JS tooling, spikes before scaffolding

The project is a Windows-only Rust program (ViGEmBus + HidHide) plus a native Kotlin Android app talking over UDP. The two halves share no code, only the wire format. So one repository holds `desktop/` (a Cargo workspace) and `mobile/` (a Gradle project with the standard `:app` module), and each side uses its own native tooling. There is no orchestrator on top. The only shared contract is the UDP protocol, which will be documented in `docs/` when R1 is specified.

Scaffolding waits for the phase 0 spikes (PRD Q1–Q7), because their answers decide how R3, R8 and R11 are built (the GUI is settled: egui, see PRD Q7). Spike code lives in `spikes/qN-<slug>/`. Each spike is self-contained (its own `Cargo.toml`, outside the future workspace) and has a README that records the answer.

## Decisions

- **Rust**: the toolchain is pinned with `rust-toolchain.toml` to the current stable release. Pure logic (protocol, mixer) builds and tests on any OS. Windows-only code sits behind `cfg(windows)`. Day-to-day development and tests run on Linux, and CI on `windows-latest` proves the `.exe` builds. Running against real drivers happens on the Windows machine.
- **Android**: `minSdk 26`; `compileSdk`/`targetSdk` = latest stable. Compose requires API 23 or later, and `TYPE_GAME_ROTATION_VECTOR` exists since API 18.
- **Quality gates**: CI only (fmt, clippy, tests on the Rust side; ktlint/lint and tests on the Android side). No local git hooks.

## Considered Options

- **Turborepo / pnpm / Node**: rejected. No JavaScript is shipped, and Cargo and Gradle already manage their own workspaces, so an orchestrator would add a toolchain and gain nothing.
- **Docker / docker-compose / database / `/health` walking skeleton**: rejected. ViGEmBus and HidHide are Windows kernel drivers and cannot run in a container, and an Android app does not run in Docker either. There is no server and no database: profiles are TOML files.
- **Husky + lint-staged**: rejected because they require Node. Shell hooks and lefthook were also declined in favour of CI-only checks, to keep setup at zero steps per clone.
- **Two repositories**: rejected. A single solo maintainer would pay the coordination cost and get no benefit.
- **Local cross-compilation to Windows (cargo-xwin / windows-gnu)**: deferred. Windows CI covers compilation. Add cross-compilation only if waiting for CI becomes a bottleneck.
