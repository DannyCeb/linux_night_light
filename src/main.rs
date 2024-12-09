use chrono::{Local, Timelike};
use std::{env, process::Command, thread, time::Duration};

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

    // Validate the night_start and night_end hour values to ensure they are within the range 0 to 23
    if night_start > 23 || night_end > 23 {
        eprintln!("Error: Night start and end hours must be between 0 and 23.");
        std::process::exit(1);
    }

    // Boolean flags to track if night or day gamma was set
    let mut bool_night = true;
    let mut bool_day = true;

    loop {
        // Get the current hour
        let hour = Local::now().hour();

        // Check if the current hour is within the night range and night gamma needs to be set
        if (hour >= night_start || hour <= night_end) && bool_night {
            // Set the night gamma
            Command::new("xrandr")
                .arg("--output")
                .arg(monitor)
                .arg("--gamma")
                .arg(night_gamma)
                .output()
                .expect("failed to execute process");

            // Update flags
            bool_night = false;
            bool_day = true;
        // Check if day gamma needs to be set
        } else if bool_day {
            // Set the day gamma
            Command::new("xrandr")
                .arg("--output")
                .arg(monitor)
                .arg("--gamma")
                .arg(day_gamma)
                .output()
                .expect("failed to execute process");

            // Update flags
            bool_day = false;
            bool_night = true;
        }

        // Sleep for 60 seconds before checking again
        thread::sleep(Duration::from_secs(60));
    }
}
