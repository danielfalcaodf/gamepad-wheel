# gamepad-wheel

Use your **Android phone as a steering wheel** together with a **GameSir Nova 2 Lite**, and have games see a **single virtual Xbox 360 controller**.

Clip the phone to the gamepad: tilting it steers (gyroscope, ~±90°), while the gamepad handles throttle, brake, camera, indicators and everything else. Free, no ads.

- **PC (Windows):** Rust app using ViGEmBus (virtual controller) and HidHide (hides the physical pad).
- **Phone (Android):** native Kotlin app streaming the rotation over UDP on Wi-Fi, paired via QR code.
- **Targets:** ETS2/ATS, Forza/NFS, Assetto Corsa/BeamNG/F1, GTA.

## Status

Phase 0: technical spikes. The skeleton builds, but there are no features yet. Product requirements: [docs/PRD.md](docs/PRD.md) (Portuguese). Decisions: [docs/adr/](docs/adr/).

## Repository layout

```text
desktop/              Cargo workspace: the Windows program (gamepad-wheel.exe)
mobile/               Gradle project: the Android app (module :app)
spikes/               Phase 0 throwaway programs, one per PRD open question
scripts/check.sh      Runs every CI check locally
rust-toolchain.toml   Rust version pin shared by desktop/ and spikes/
docs/                 PRD, ADRs, agent docs
```

## Development

**Prerequisites**
- Docker. Rust runs in the official `rust` image, so no host Rust install is needed.
- JDK 17+ and an Android SDK whose directory is writable, so Gradle can fetch platform 37. Point `mobile/local.properties` at it: `sdk.dir=/path/to/Android/Sdk`.
- Windows 10/11 with [ViGEmBus](https://github.com/nefarius/ViGEmBus) 1.17+ and [HidHide](https://github.com/nefarius/HidHide), to run the PC program and the spikes.

**Commands**

```sh
scripts/check.sh            # everything CI runs: fmt, clippy, tests, Android lint/test/build
scripts/check.sh desktop    # only the Rust workspace
scripts/check.sh spikes     # only the spikes
scripts/check.sh mobile     # only Android
cd mobile && ./gradlew installDebug   # install the app on a connected phone
gh run download --name spikes-windows # spike .exe files built by CI
```

Linux can type-check the Windows code but cannot link it. CI builds the real `.exe` on `windows-latest`.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
