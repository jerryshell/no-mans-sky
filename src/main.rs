use std::sync::{Arc, Mutex};

use no_mans_sky::*;

fn main() {
    let task = Arc::new(Mutex::new(eth_task::ETHTask));
    run(task)
}
