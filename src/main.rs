use expanduser::expanduser;
use notify_rust::{Hint, Notification, Timeout};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn reload_sway() {
    let sway_restart = Command::new("swaymsg").arg("reload").output();
    match sway_restart {
        Ok(_ok) => println!("Successfully reloaded Sway."),
        Err(_err) => println!("Could not reload sway."),
    };
}

fn main() {
    let config_path = expanduser("~/.config/sway/config").expect("Could not expand path");
    let data = std::fs::read_to_string(&config_path).expect("No can do");
    if data.contains("dwt enabled") {
        let new_data = data.replace("dwt enabled", "dwt disabled");
        let new_file = File::create(config_path);
        let write_result = new_file
            .expect("Could not make file")
            .write_all(new_data.as_bytes());

        match write_result {
            Ok(_ok) => {
                println!("Successfully wrote file.");
                reload_sway()
            }
            Err(_err) => println!("Could not write file."),
        }
        println!("Palm detection is now: DISABLED.");
        Notification::new()
            .summary("DISABLED")
            .body("Palm detection switched.")
            .icon("dialog-information")
            .appname("palm-detection-switch")
            .hint(Hint::Category("Device".to_owned()))
            .timeout(Timeout::Milliseconds(10000))
            .show()
            .unwrap();
    } else if data.contains("dwt disabled") {
        let new_data = data.replace("dwt disabled", "dwt enabled");
        let new_file = File::create(config_path);
        let write_result = new_file
            .expect("Could not make file")
            .write_all(new_data.as_bytes());

        match write_result {
            Ok(_ok) => {
                println!("Successfully wrote file.");
                reload_sway()
            }
            Err(_err) => println!("Could not write file."),
        }
        println!("Palm detection is now: ENABLED.");
        Notification::new()
            .summary("ENABLED")
            .body("Palm detection switched.")
            .icon("dialog-information")
            .appname("palm-detection-switch")
            .hint(Hint::Category("Device".to_owned()))
            .timeout(Timeout::Milliseconds(10000))
            .show()
            .unwrap();
    }
}
