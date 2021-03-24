use std::thread;
use std::time::Duration;

use chrono::Utc;

use no_mans_sky::*;

const PID_INIT_VALUE: u32 = 0;
const USER_COUNT_THRESHOLD: usize = 0;

fn main() {
    let task = ETHTask;
    run(&task);
}

fn run<T: Task>(task: &T) {
    let mut pid: u32 = PID_INIT_VALUE;
    loop {
        match get_user_count() {
            Some(user_count) => {
                println!("{} user_count: {}", Utc::now(), user_count);
                if user_count > USER_COUNT_THRESHOLD && pid != PID_INIT_VALUE {
                    clean(task, &mut pid);
                }
                if user_count <= USER_COUNT_THRESHOLD && pid == PID_INIT_VALUE {
                    match task.init_env() {
                        Ok(_) => {
                            pid = match task.start_target_process() {
                                Some(pid) => {
                                    println!("target process pid: {}", pid);
                                    thread::sleep(Duration::from_secs(10));
                                    pid
                                }
                                None => PID_INIT_VALUE,
                            };
                            if let Some(pid) = task.start_target_process() {
                                println!("target process pid: {}", pid);
                                thread::sleep(Duration::from_secs(10));
                            }
                        }
                        Err(err) => {
                            println!("init env error: {}", err);
                        }
                    };
                    task.clean_env();
                }
            }
            None => clean(task, &mut pid),
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn clean<T: Task>(task: &T, pid: &mut u32) {
    task.clean_env();
    if *pid != PID_INIT_VALUE {
        task.kill_target_process(pid);
        *pid = PID_INIT_VALUE;
    }
}
