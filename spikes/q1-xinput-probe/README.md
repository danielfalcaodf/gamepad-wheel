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

<!-- Fill in after running: Q1 yes/no (+ step that failed), Q2 held/pulsed, chord used, firmware version. -->
