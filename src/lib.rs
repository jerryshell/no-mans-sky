use chrono::Utc;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub mod eth_task;
pub mod task;

const PID_INIT_VALUE: u32 = 0;
const USER_COUNT_THRESHOLD: usize = 0;

pub fn run<T: task::Task + Sync>(task: &'static T) {
    let mut pid: u32 = PID_INIT_VALUE;
    loop {
        match get_user_count() {
            Ok(user_count) => {
                println!("{} user_count: {}", Utc::now(), user_count);
                if user_count > USER_COUNT_THRESHOLD && pid != PID_INIT_VALUE {
                    clean(task, &mut pid);
                }
                if user_count == USER_COUNT_THRESHOLD && pid == PID_INIT_VALUE {
                    match task.init_env() {
                        Ok(_) => {
                            pid = match task.start_target_process() {
                                Ok(pid) => pid,
                                Err(e) => {
                                    println!("start target process error: {}", e);
                                    PID_INIT_VALUE
                                }
                            };
                        }
                        Err(e) => {
                            println!("init env error: {}", e);
                        }
                    }
                    println!("target process pid: {}", pid);
                    // spawn clean_env thread
                    thread::spawn(move || {
                        if pid != PID_INIT_VALUE {
                            thread::sleep(Duration::from_secs(10));
                        }
                        if let Err(e) = task.clean_env() {
                            println!("clean env error: {}", e);
                        }
                    });
                }
            }
            Err(e) => {
                println!("get user count error: {}", e);
                clean(task, &mut pid);
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

pub fn get_user_count() -> anyhow::Result<usize> {
    let w_output = Command::new("w").arg("-h").output()?;
    Ok(String::from_utf8_lossy(&w_output.stdout).lines().count())
}

fn clean<T: task::Task>(task: &T, pid: &mut u32) {
    if let Err(e) = task.clean_env() {
        println!("clean env error: {}", e);
    }
    if *pid != PID_INIT_VALUE {
        if let Err(e) = task.kill_target_process(pid) {
            println!("kill target process error: {}", e);
        };
        *pid = PID_INIT_VALUE;
    }
}
