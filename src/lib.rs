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
                                Some(pid) => pid,
                                None => PID_INIT_VALUE,
                            };
                        }
                        Err(err) => {
                            println!("init env error: {}", err);
                        }
                    }
                    println!("target process pid: {}", pid);
                    // spawn clean_env thread
                    thread::spawn(move || {
                        if pid != PID_INIT_VALUE {
                            thread::sleep(Duration::from_secs(10));
                        }
                        task.clean_env();
                    });
                }
            }
            Err(_) => clean(task, &mut pid),
        }
        thread::sleep(Duration::from_secs(1));
    }
}

pub fn get_user_count() -> anyhow::Result<usize> {
    let w_output = Command::new("w").arg("-h").output()?;
    Ok(String::from_utf8_lossy(&w_output.stdout).lines().count())
}

fn clean<T: task::Task>(task: &T, pid: &mut u32) {
    task.clean_env();
    if *pid != PID_INIT_VALUE {
        task.kill_target_process(pid);
        *pid = PID_INIT_VALUE;
    }
}
