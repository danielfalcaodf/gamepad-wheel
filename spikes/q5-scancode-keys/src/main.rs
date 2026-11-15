//! Spike Q5: after a delay (time to focus the game), press and release one
//! key through `SendInput` using a hardware scan code, not a virtual key.
//!
//! Usage: q5-scancode-keys <scancode-hex> [delay-s=5] [hold-ms=100]
//!   scan code is hex, e.g. 26 (L); prefix e0 for extended keys, e.g. e04b (Left arrow).

/// Parses "26" -> (0x26, false) and "e04b" -> (0x4b, true).
fn parse_scancode(s: &str) -> Option<(u16, bool)> {
    let s = s.trim_start_matches("0x").to_ascii_lowercase();
    match s.strip_prefix("e0") {
        Some(rest) if !rest.is_empty() => u16::from_str_radix(rest, 16).ok().map(|c| (c, true)),
        _ => u16::from_str_radix(&s, 16).ok().map(|c| (c, false)),
    }
}

#[cfg(not(windows))]
fn main() {
    eprintln!("Windows only.");
}

#[cfg(windows)]
fn main() {
    use std::time::Duration;
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((code, extended)) = args.first().and_then(|a| parse_scancode(a)) else {
        eprintln!("usage: q5-scancode-keys <scancode-hex> [delay-s=5] [hold-ms=100]");
        eprintln!("  e.g. 26 = L (ETS2 headlights), e04b = Left arrow");
        std::process::exit(2);
    };
    let delay: u64 = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(5);
    let hold: u64 = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(100);

    let key = |up: bool| {
        let mut flags = KEYEVENTF_SCANCODE;
        if extended {
            flags |= KEYEVENTF_EXTENDEDKEY;
        }
        if up {
            flags |= KEYEVENTF_KEYUP;
        }
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wScan: code,
                    dwFlags: flags,
                    ..Default::default()
                },
            },
        };
        let sent = unsafe { SendInput(&[input], size_of::<INPUT>() as i32) };
        assert_eq!(
            sent, 1,
            "SendInput was blocked (UIPI? run as the same user/level as the game)"
        );
    };

    println!("Focus the game. Sending scan code {code:#04x} (extended={extended}) in {delay} s...");
    std::thread::sleep(Duration::from_secs(delay));
    key(false);
    std::thread::sleep(Duration::from_millis(hold));
    key(true);
    println!("Sent (held {hold} ms).");
}

#[cfg(test)]
mod tests {
    use super::parse_scancode;

    #[test]
    fn parses_plain_and_extended_codes() {
        assert_eq!(parse_scancode("26"), Some((0x26, false)));
        assert_eq!(parse_scancode("0x1A"), Some((0x1a, false)));
        assert_eq!(parse_scancode("e04b"), Some((0x4b, true)));
        assert_eq!(parse_scancode("zz"), None);
    }
}
