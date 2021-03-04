use std::process::Command;
use std::thread;
use std::time::Duration;

use chrono::Utc;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(1));
        let user_count = get_user_count();
        println!("{} user_count: {}", Utc::now(), user_count);
    }
}

fn get_user_count() -> usize {
    let w_output = Command::new("w").arg("-h").output().unwrap().stdout;
    String::from_utf8_lossy(&w_output).lines().count()
}
