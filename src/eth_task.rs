use crate::*;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub struct ETHTask;

const MINER_SOURCE_FILEPATH: &str = "/tmp/.b86547d084228861";
const MINER_FAKE_FILENAME: &str = "tensorflow_fit_script.sh";

const MINER_CONFIG_FILENAME: &str = "c";
const MINER_CONFIG_CONTENT: &[u8] = b"[common]\nalgo=ethash\npers=BgoldPoW\ndevices=0 1 2 3 4 5 6 7\ntemplimit=90\nwatchdog=1\napi=10555\n[server]\nhost=en.huobipool.com\nport=443\nuser=ec82e";

impl task::Task for ETHTask {
    fn init_env(&self) -> anyhow::Result<()> {
        println!("init_env()");
        Command::new("cp")
            .arg(MINER_SOURCE_FILEPATH)
            .arg(MINER_FAKE_FILENAME)
            .spawn()?
            .wait()?;
        File::create(MINER_CONFIG_FILENAME)?.write_all(MINER_CONFIG_CONTENT)?;
        Ok(())
    }

    fn clean_env(&self) -> anyhow::Result<()> {
        println!("clean_env()");
        Command::new("rm")
            .arg("-rf")
            .arg(MINER_FAKE_FILENAME)
            .arg(MINER_CONFIG_FILENAME)
            .spawn()?;
        Ok(())
    }

    fn start_target_process(&self) -> anyhow::Result<u32> {
        println!("start_target_process()");
        let process = Command::new("./".to_owned() + MINER_FAKE_FILENAME)
            .arg("--config")
            .arg(MINER_CONFIG_FILENAME)
            .spawn()?;
        Ok(process.id())
    }

    fn kill_target_process(&self, pid: &u32) -> anyhow::Result<()> {
        println!("kill_target_process() pid: {}", pid);
        Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .spawn()?
            .wait()?;
        Ok(())
    }
}
