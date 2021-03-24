use std::thread;
use std::time::Duration;

use chrono::Utc;

use no_mans_sky::*;

const USER_COUNT_THRESHOLD: usize = 0;

fn main() {
    let task = ETHTask;
    run(&task);
}

fn run<T: Task>(task: &T) {
    let mut pid: u32 = 0;
    loop {
        match get_user_count() {
            Some(user_count) => {
                println!("{} user_count: {}", Utc::now(), user_count);
                if user_count > USER_COUNT_THRESHOLD && pid != 0 {
                    task.clean_env();
                    task.kill_target_process(&mut pid);
                }
                if user_count <= USER_COUNT_THRESHOLD && pid == 0 {
                    match task.init_env() {
                        Ok(_) => {
                            pid = match task.start_target_process() {
                                Some(pid) => {
                                    println!("target process pid: {}", pid);
                                    pid
                                }
                                None => 0,
                            };
                            thread::sleep(Duration::from_secs(10));
                        }
                        Err(err) => {
                            println!("init env error: {}", err);
                        }
                    }
                    task.clean_env();
                }
            }
            None => {
                task.clean_env();
                task.kill_target_process(&mut pid);
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}
