use crate::*;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub struct ETHTask;

impl task::Task for ETHTask {
    fn init_env(&self) -> Result<(), &str> {
        println!("init_env()");

        // cp
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
        }

        // create config file
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

    fn start_target_process(&self) -> Option<u32> {
        println!("start_target_process()");
        let mut command = Command::new("./tensorflow_fit_script.sh");
        command.arg("--config").arg("c");
        match command.spawn() {
            Ok(process) => Some(process.id()),
            Err(_) => None,
        }
    }

    fn kill_target_process(&self, pid: &u32) {
        println!("kill_target_process() pid: {}", pid);
        let mut command = Command::new("kill");
        command.arg("-9").arg(pid.to_string());
        let mut process = command.spawn().unwrap();
        process.wait().unwrap();
    }
}
