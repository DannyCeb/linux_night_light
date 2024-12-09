use std::{process::Command, thread, time::Duration};

use chrono::{Local, Timelike};

fn main() {
    loop {
        let hour = Local::now().hour();

        if hour >= 19 || hour <= 7 {
            Command::new("xrandr")
                .arg("--output")
                .arg("eDP")
                .arg("--gamma")
                .arg("1.0:0.8:0.7")
                .output()
                .expect("failed to execute process");
        } else {
            Command::new("xrandr")
                .arg("--output")
                .arg("eDP")
                .arg("--gamma")
                .arg("1.0:1.0:1.0")
                .output()
                .expect("failed to execute process");
        }

        thread::sleep(Duration::from_secs(60));
    }
}
