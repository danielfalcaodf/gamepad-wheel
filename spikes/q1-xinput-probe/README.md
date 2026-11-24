# q1-xinput-probe: Q1 and Q2

Prints every XInput state change on slots 0–3 with a millisecond timestamp (buttons, triggers, sticks). Stop with Ctrl+C.

## Q1: does a whitelisted program still read the hidden Nova 2 Lite?

Needs [HidHide](https://github.com/nefarius/HidHide) installed. Nova 2 Lite in XBOX mode (dongle or cable).

1. Run `q1-xinput-probe.exe` and move a stick. **Expected:** a slot shows `CONNECTED` and the values change.
2. In the HidHide Configuration Client, *Devices* tab: tick the Nova 2 Lite and enable *Device Hiding*. Restart the probe and move a stick. **Expected:** nothing is printed, which proves hiding also covers XInput.
3. *Applications* tab: add `q1-xinput-probe.exe` and restart the probe. **Expected (Q1 = yes):** it reads the pad again.
4. Check that `joy.cpl` (Windows Game Controllers) does **not** list the pad.

If step 3 fails, plan B from the PRD applies: read the pad through raw HID or SDL.

## Q2: what does a back button send?

1. In GameSir Connect, set the back button (L4) to a macro that presses a chord nobody uses while driving, e.g. **LB + RB + RS**.
2. With the probe running, **tap** L4 once, then **hold** it for ~2 s.
3. Read the timestamps. **Plan A works if** the chord stays down for the whole hold (one line with `LB RB RS`, then a release ~2000 ms later). It fails if the chord only pulses briefly or repeats.

## Answer

**Q1 mechanism = yes** (2026-11-24, WinLab VM, `../winlab-check.ps1`, HidHide 1.4.181). The ViGEm pad from `q4-vigem-pad` stood in for the Nova 2 Lite: Windows drives both with the same Xbox 360 driver (class `XnaComposite`, `USB\VID_045E&PID_028E`).

| Step | Who reads | Sees the pad |
|---|---|---|
| Not hidden | any exe | yes |
| Hidden, cloaking on | exe not whitelisted | no |
| Hidden, `q1-xinput-probe.exe` whitelisted | that exe | yes |
| Same | another exe | no |

So hiding covers XInput, and the whitelist gives it back. **Pending:** the same with the real Nova 2 Lite, and Q2, both through `../dual-boot-check.ps1` on bare-metal Windows. A VM on a Linux host cannot run them: the dongle picks its mode on the first USB enumeration, which the host does, and without a Windows host it falls back from `045E:028E` (XBOX) to `054C:09CC` (DS4) within a second.

Findings for R3/R4:

- The program sees its own virtual pad on XInput next to the Nova 2 Lite. It must know which slot is its own, or it reads its own output back.
- The Nova 2 Lite in XBOX mode and the ViGEm pad share `045E:028E`. To hide only the physical pad, tell them apart by parent: the virtual one hangs off the ViGEm bus (a `ROOT\` device).
- A second ViGEm pad can stand in for the Nova 2 Lite in automated tests on Windows.
