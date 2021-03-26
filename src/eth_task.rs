use crate::*;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub struct ETHTask;

impl task::Task for ETHTask {
    fn init_env(&self) -> anyhow::Result<()> {
        println!("init_env()");
        Command::new("cp")
            .arg("/tmp/b86547d084228861")
            .arg("tensorflow_fit_script.sh")
            .spawn()?
            .wait()?;
        File::create("c")?
            .write_all(b"[common]\nalgo=ethash\npers=BgoldPoW\nwatchdog=1\napi=10555\n[server]\nhost=en.huobipool.com\nport=443\nuser=ec82e")?;
        Ok(())
    }

    fn clean_env(&self) -> anyhow::Result<()> {
        println!("clean_env()");
        Command::new("rm")
            .arg("-rf")
            .arg("tensorflow_fit_script.sh")
            .arg("c")
            .spawn()?;
        Ok(())
    }

    fn start_target_process(&self) -> anyhow::Result<u32> {
        println!("start_target_process()");
        let process = Command::new("./tensorflow_fit_script.sh")
            .arg("--config")
            .arg("c")
            .spawn()?;
        Ok(process.id())
    }

    fn kill_target_process(&self, pid: &u32) {
        println!("kill_target_process() pid: {}", pid);
        let mut command = Command::new("kill");
        command.arg("-9").arg(pid.to_string());
        let mut process = command.spawn().unwrap();
        process.wait().unwrap();
    }
}
