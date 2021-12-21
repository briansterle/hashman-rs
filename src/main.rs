use std::thread::sleep;
use std::time::Duration;

use log::debug;

use hashman_rs::{get_arg_or, HashEnv, Rig};

fn main() {
  env_logger::init();
  let loops: u64 = get_arg_or(1, |s| s.parse::<u64>().expect("loops: int"), 21_000_000);
  let interval: Duration = get_arg_or(
    2,
    |sec| Duration::from_secs(sec.parse::<u64>().expect("duration: sec")),
    Duration::from_secs(21),
  );
  let mut env = HashEnv::setup();
  for _ in 1..loops {
    let updated: Rig = env.run();
    debug!("Rig::move_state = {:?}", updated);
    sleep(interval);
  }
}
