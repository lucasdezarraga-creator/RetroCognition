use active_win_pos_rs::get_active_window;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Initializting...");

    loop {
        match get_active_window() {
            Ok(window) => {
                println!("Active App: {} | Window Title: {}", window.app_name, window.title);
            }

            Err(_) => {
                println!("Could not detect an active window right now.");
            }
        }
        sleep(Duration::from_secs(2));
    }
}
