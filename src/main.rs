use std::{env, process::Command, thread, time::Duration};

use chrono::{Local, Timelike};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Verify that the correct parameters were passed
    if args.len() != 6 {
        eprintln!(
            "Usage: {} <day_gamma> <night_gamma> <night_start_hour> <night_end_hour> <monitor>",
            args[0]
        );
        std::process::exit(1);
    }

    // Parse the arguments
    let day_gamma = &args[1];
    let night_gamma = &args[2];
    let night_start: u32 = args[3].parse().expect("Invalid night start hour");
    let night_end: u32 = args[4].parse().expect("Invalid night end hour");
    let monitor = &args[5];

    loop {
        let hour = Local::now().hour();

        if hour >= night_start || hour <= night_end {
            Command::new("xrandr")
                .arg("--output")
                .arg(monitor)
                .arg("--gamma")
                .arg(night_gamma)
                .output()
                .expect("failed to execute process");
        } else {
            Command::new("xrandr")
                .arg("--output")
                .arg(monitor)
                .arg("--gamma")
                .arg(day_gamma)
                .output()
                .expect("failed to execute process");
        }

        thread::sleep(Duration::from_secs(60));
    }
}
