pub trait Task {
    fn init_env(&self) -> Result<(), &str>;
    fn clean_env(&self);
    fn start_target_process(&self) -> Option<u32>;
    fn kill_target_process(&self, pid: &u32);
}
