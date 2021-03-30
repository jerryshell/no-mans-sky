pub trait Task: Send {
    fn init_env(&self) -> anyhow::Result<()>;
    fn clean_env(&self) -> anyhow::Result<()>;
    fn start_target_process(&self) -> anyhow::Result<u32>;
    fn kill_target_process(&self, pid: &u32) -> anyhow::Result<()>;
}
