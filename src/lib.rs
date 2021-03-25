use chrono::Utc;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub mod eth_task;
pub mod task;

const PID_INIT_VALUE: u32 = 0;
const USER_COUNT_THRESHOLD: usize = 0;

pub fn run<T: task::Task>(task: &T) {
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
                            }
                        }
                        Err(err) => {
                            println!("init env error: {}", err);
                        }
                    }
                    task.clean_env();
                }
            }
            None => clean(task, &mut pid),
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_user_count() -> Option<usize> {
    match Command::new("w").arg("-h").output() {
        Ok(output) => Some(String::from_utf8_lossy(&output.stdout).lines().count()),
        Err(_) => None,
    }
}

fn clean<T: task::Task>(task: &T, pid: &mut u32) {
    task.clean_env();
    if *pid != PID_INIT_VALUE {
        task.kill_target_process(pid);
        *pid = PID_INIT_VALUE;
    }
}
