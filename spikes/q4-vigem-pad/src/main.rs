//! Spike Q4 (+ R14): plug a virtual Xbox 360 pad through ViGEmBus with
//! `vigem-rust`, sweep its left stick for 60 s, and print every rumble
//! command the host sends back.

#[cfg(not(windows))]
fn main() {
    eprintln!("Windows only.");
}

#[cfg(windows)]
fn main() {
    use std::time::{Duration, Instant};
    use vigem_rust::{Client, X360Report};

    let client = match Client::connect() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not reach ViGEmBus: {e:?}");
            eprintln!("Install ViGEmBus 1.17+ (Windows 10/11) and run again.");
            std::process::exit(1);
        }
    };
    let pad = client
        .new_x360_target()
        .plug()
        .and_then(|p| p.wait_for_ready())
        .expect("plug virtual Xbox 360 pad");

    let notifications = pad.register_notification().expect("register rumble");
    let start = Instant::now();
    std::thread::spawn(move || {
        while let Ok(n) = notifications.recv() {
            println!(
                "{:>8} ms  rumble large={:3} small={:3} led={}",
                start.elapsed().as_millis(),
                n.large_motor,
                n.small_motor,
                n.led_number
            );
        }
    });

    println!("Virtual Xbox 360 pad plugged. Left stick X sweeps for 60 s (Ctrl+C to stop).");
    println!("Check joy.cpl and a rumble test (e.g. a gamepad tester web page).");
    let mut report = X360Report::default();
    while start.elapsed() < Duration::from_secs(60) {
        let t = start.elapsed().as_secs_f32();
        report.thumb_lx = (t.sin() * f32::from(i16::MAX)) as i16;
        pad.update(&report).expect("send report");
        std::thread::sleep(Duration::from_millis(4)); // 250 Hz, the R4 target
    }
    println!("Done; pad unplugs on exit.");
}
