use std::{
    env,
    sync::{Arc, Mutex},
};

use no_mans_sky::*;

fn main() {
    let kill_at = env::args().nth(1);
    let task = Arc::new(Mutex::new(eth_task::ETHTask));
    match kill_at {
        Some(kill_at_str) => run(task, Some(kill_at_str.parse::<u64>().unwrap())),
        None => run(task, None),
    }
}
