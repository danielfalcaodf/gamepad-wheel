---
status: accepted
---

# Keep ViGEmBus despite its retirement, via `vigem-rust`, behind our own seam

ViGEmBus was archived in November 2023 and will get no more fixes. Version 1.17+ still works on Windows 10/11, and in 2026 widely used projects (Sunshine) still depend on it because there is no free, stable replacement that emulates an Xbox 360 pad. We use it anyway, and accept the risk that a future Windows update breaks it with no fix coming. The Rust code talks to it only through one module of ours (the virtual pad), so swapping the driver touches a single place.

The Rust binding is `vigem-rust` (0.2.0, Aug 2026, MIT OR Apache-2.0, pure Rust), not `vigem-client` as the PRD first said. `vigem-client` (0.1.4, 2022) cannot receive the rumble the game sends to the virtual pad, which R14 needs. `vigem-rust` exposes it through `register_notification()`. The trade-off is that `vigem-rust` is young and lightly used, and the same seam limits that risk.

## Considered Options

- **LizardByte Virtual HID Driver**: user-mode, but creating gamepads on Windows needs a paid license, and it offers no Xbox 360 type. Its first release ships with a beta of Sunshine (Aug 2026).
- **VIIPER (DS4Windows 5)**: still in release candidates, depends on the usbip-win2 kernel driver, and its upstream repo was removed from GitHub.
- **Revisit when** Windows breaks ViGEmBus, or when a free, signed successor with Xbox 360 emulation becomes stable.
