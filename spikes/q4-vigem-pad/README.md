# q4-vigem-pad: Q4 and the rumble half of R14

Plugs a virtual Xbox 360 pad through ViGEmBus using `vigem-rust`, sweeps its left stick X for 60 s at 250 Hz, and prints every rumble command a game sends to it.

Needs ViGEmBus 1.17+ installed. The program says so if it's missing.

1. Run `q4-vigem-pad.exe`.
2. Open `joy.cpl`. **Expected:** a new "Controller (XBOX 360 For Windows)" whose X axis sweeps left and right.
3. Trigger vibration on the virtual pad, either from a gamepad tester page in the browser or from a game. **Expected:** lines like `rumble large=... small=...`.

Q4 = yes if steps 2 and 3 both work on your Windows version.

## Answer

<!-- Fill in: Windows build (winver), ViGEmBus version, pad visible yes/no, rumble received yes/no. -->
