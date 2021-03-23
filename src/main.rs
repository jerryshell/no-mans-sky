use std::thread;
use std::time::Duration;

use chrono::Utc;

use no_mans_sky::*;

const USER_COUNT_THRESHOLD: usize = 0;

fn main() {
    let mut pid: u32 = 0;
    let task = ETHTask;
    loop {
        let user_count = get_user_count();
        println!("{} user_count: {}", Utc::now(), user_count);
        if user_count > USER_COUNT_THRESHOLD && pid != 0 {
            task.clean_env();
            task.kill_target_process(&mut pid);
        }
        if user_count <= USER_COUNT_THRESHOLD && pid == 0 {
            task.init_env();
            pid = task.start_target_process();
            thread::sleep(Duration::from_secs(10));
            task.clean_env();
        }
        thread::sleep(Duration::from_secs(1));
    }
}
