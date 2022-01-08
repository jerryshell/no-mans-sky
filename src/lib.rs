use std::{
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub mod eth_task;
pub mod task;

const PID_INIT_VALUE: u32 = 0;
const USER_COUNT_THRESHOLD: usize = 0;

pub fn get_user_count() -> anyhow::Result<usize> {
    let w_output = Command::new("w").arg("-h").output()?;
    Ok(String::from_utf8_lossy(&w_output.stdout).lines().count())
}

pub fn run(task_arc: Arc<Mutex<dyn task::Task>>, kill_at_unix_timestamp_secs: i64) {
    let mut pid: u32 = PID_INIT_VALUE;
    let running_flag = Arc::new(AtomicBool::new(true));

    // init SIGINT, SIGTERM handler
    let running_flag_clone = running_flag.clone();
    ctrlc::set_handler(move || {
        running_flag_clone.store(false, Ordering::SeqCst);
    })
    .unwrap();

    while running_flag.load(Ordering::SeqCst) {
        if kill_at_unix_timestamp_secs > 0 {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(current) => {
                    if current.as_secs() >= kill_at_unix_timestamp_secs.try_into().unwrap() {
                        break;
                    }
                }
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            }
        }
        match get_user_count() {
            Ok(user_count) => {
                println!("{} user_count: {}", chrono::Utc::now(), user_count);

                if user_count > USER_COUNT_THRESHOLD && pid != PID_INIT_VALUE {
                    clean(task_arc.clone(), &mut pid);
                }

                if user_count == USER_COUNT_THRESHOLD && pid == PID_INIT_VALUE {
                    let task = task_arc.lock().unwrap();
                    match task.init_env() {
                        Err(e) => {
                            println!("init env error: {}", e);
                        }
                        Ok(_) => {
                            pid = match task.start_target_process() {
                                Ok(pid) => pid,
                                Err(e) => {
                                    println!("start target process error: {}", e);
                                    PID_INIT_VALUE
                                }
                            };
                        }
                    }
                    println!("target process pid: {}", pid);

                    // spawn clean_env thread
                    let task_arc_clone = task_arc.clone();
                    thread::spawn(move || {
                        if pid != PID_INIT_VALUE {
                            thread::sleep(Duration::from_secs(10));
                        }
                        if let Err(e) = task_arc_clone.lock().unwrap().clean_env() {
                            println!("clean env error: {}", e);
                        }
                    });
                }
            }
            Err(e) => {
                println!("get user count error: {}", e);
                clean(task_arc.clone(), &mut pid);
            }
        }
        thread::sleep(Duration::from_secs(1));
    }

    // SIGINT, SIGTERM
    clean(task_arc.clone(), &mut pid);
}

fn clean(task_arc: Arc<Mutex<dyn task::Task>>, pid: &mut u32) {
    if let Err(e) = task_arc.lock().unwrap().clean_env() {
        println!("clean env error: {}", e);
    }
    if *pid != PID_INIT_VALUE {
        if let Err(e) = task_arc.lock().unwrap().kill_target_process(pid) {
            println!("kill target process error: {}", e);
        };
        *pid = PID_INIT_VALUE;
    }
}
