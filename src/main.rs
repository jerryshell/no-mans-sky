use std::thread;
use std::time::Duration;

use chrono::Utc;

use no_mans_sky::*;

fn main() {
    let mut pid: u32 = 0;
    loop {
        let user_count = get_user_count();
        println!("{} user_count: {}", Utc::now(), user_count);
        if user_count > 0 && pid != 0 {
            clean_env();
            kill_target_process(&mut pid);
        }
        if user_count <= 0 && pid == 0 {
            init_env();
            pid = start_target_process();
            clean_env();
        }
        thread::sleep(Duration::from_secs(1));
    }
}
