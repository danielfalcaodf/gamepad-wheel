# gamepad-wheel

Use your **Android phone as a steering wheel** together with a **GameSir Nova 2 Lite**, and have games see a **single virtual Xbox 360 controller**.

Clip the phone to the gamepad: tilting it steers (gyroscope, ~±90°), while the gamepad handles throttle, brake, camera, indicators and everything else. Free, no ads.

- **PC (Windows):** Rust app using ViGEmBus (virtual controller) and HidHide (hides the physical pad).
- **Phone (Android):** native Kotlin app streaming the rotation over UDP on Wi-Fi, paired via QR code.
- **Targets:** ETS2/ATS, Forza/NFS, Assetto Corsa/BeamNG/F1, GTA.

## Status

Phase 0: technical spikes. No code yet. See the product requirements in [docs/PRD.md](docs/PRD.md) (Portuguese).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
