use clap::Parser;
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long, default_value_t = -1)]
    kill_at_unix_timestamp_secs: i64,
}

fn main() {
    let args = Args::parse();
    let task = Arc::new(Mutex::new(no_mans_sky::eth_task::ETHTask));
    no_mans_sky::run(task, args.kill_at_unix_timestamp_secs);
}
