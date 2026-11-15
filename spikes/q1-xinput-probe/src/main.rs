//! Spike Q1 + Q2: print every XInput state change on all 4 slots, with a
//! millisecond timestamp, until Ctrl+C.
//! Q1: does a HidHide-whitelisted exe still see the hidden Nova 2 Lite?
//! Q2: what does the PC receive when a back button (macro) is pressed/held?

#[cfg(not(windows))]
fn main() {
    eprintln!("Windows only.");
}

#[cfg(windows)]
fn main() {
    use std::time::{Duration, Instant};
    use windows::Win32::UI::Input::XboxController::*;

    const NAMES: [(XINPUT_GAMEPAD_BUTTON_FLAGS, &str); 14] = [
        (XINPUT_GAMEPAD_DPAD_UP, "UP"),
        (XINPUT_GAMEPAD_DPAD_DOWN, "DOWN"),
        (XINPUT_GAMEPAD_DPAD_LEFT, "LEFT"),
        (XINPUT_GAMEPAD_DPAD_RIGHT, "RIGHT"),
        (XINPUT_GAMEPAD_START, "START"),
        (XINPUT_GAMEPAD_BACK, "BACK"),
        (XINPUT_GAMEPAD_LEFT_THUMB, "LS"),
        (XINPUT_GAMEPAD_RIGHT_THUMB, "RS"),
        (XINPUT_GAMEPAD_LEFT_SHOULDER, "LB"),
        (XINPUT_GAMEPAD_RIGHT_SHOULDER, "RB"),
        (XINPUT_GAMEPAD_A, "A"),
        (XINPUT_GAMEPAD_B, "B"),
        (XINPUT_GAMEPAD_X, "X"),
        (XINPUT_GAMEPAD_Y, "Y"),
    ];
    const ERROR_SUCCESS: u32 = 0;

    println!("q1-xinput-probe: polling XInput slots 0-3 every 4 ms. Ctrl+C to stop.");
    let start = Instant::now();
    // None = disconnected; Some(packet number) = last state printed.
    let mut last: [Option<u32>; 4] = [None; 4];

    loop {
        for slot in 0..4u32 {
            let mut st = XINPUT_STATE::default();
            let connected = unsafe { XInputGetState(slot, &mut st) } == ERROR_SUCCESS;
            let ms = start.elapsed().as_millis();
            let prev = &mut last[slot as usize];
            match (connected, *prev) {
                (false, Some(_)) => {
                    println!("{ms:>8} ms  slot {slot}: DISCONNECTED");
                    *prev = None;
                }
                (true, p) if p != Some(st.dwPacketNumber) => {
                    if p.is_none() {
                        println!("{ms:>8} ms  slot {slot}: CONNECTED");
                    }
                    let g = st.Gamepad;
                    let pressed: Vec<&str> = NAMES
                        .iter()
                        .filter(|(f, _)| g.wButtons.0 & f.0 != 0)
                        .map(|(_, n)| *n)
                        .collect();
                    println!(
                        "{ms:>8} ms  slot {slot}: buttons=0x{:04X} [{}]  LT={:3} RT={:3}  L=({:6},{:6}) R=({:6},{:6})",
                        g.wButtons.0,
                        pressed.join(" "),
                        g.bLeftTrigger,
                        g.bRightTrigger,
                        g.sThumbLX,
                        g.sThumbLY,
                        g.sThumbRX,
                        g.sThumbRY,
                    );
                    *prev = Some(st.dwPacketNumber);
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(4));
    }
}
