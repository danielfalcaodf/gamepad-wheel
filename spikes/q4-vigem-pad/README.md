# q4-vigem-pad: Q4 and the rumble half of R14

Plugs a virtual Xbox 360 pad through ViGEmBus using `vigem-rust`, sweeps its left stick X for 60 s at 250 Hz, and prints every rumble command a game sends to it.

Needs ViGEmBus 1.17+ installed. The program says so if it's missing.

1. Run `q4-vigem-pad.exe`.
2. Open `joy.cpl`. **Expected:** a new "Controller (XBOX 360 For Windows)" whose X axis sweeps left and right.
3. Trigger vibration on the virtual pad, either from a gamepad tester page in the browser or from a game. **Expected:** lines like `rumble large=... small=...`.

Q4 = yes if steps 2 and 3 both work on your Windows version.

## Answer

**Q4 = yes** (2026-11-24, WinLab VM, checked by `../winlab-check.ps1`).

| Item | Result |
|---|---|
| Windows | 11 26H2, build 10.0.26300 |
| ViGEmBus | 1.21.442 |
| Pad visible | yes: "Controlador XBOX 360 para Windows", `USB\VID_045E&PID_028E`, XInput slot 0 |
| Stick sweep read back through XInput | yes: about 200 state changes per second in `q1-xinput-probe` |
| Rumble received | yes: `XInputSetState` left 65535 arrived as `large=255`, right 32768 as `small=128` |

Not covered here: rumble from a real game, and the Nova 2 Lite vibrating (see `../dual-boot-check.ps1`).
