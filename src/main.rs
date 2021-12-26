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
    Duration::from_secs(10),
  );
  let mut env = HashEnv::setup();

  let run_then_sleep = |env: &mut HashEnv, is_deep_search: bool| {
    let updated: Rig = env.run(is_deep_search);
    debug!("Rig::move_state = {:?}", updated);
    sleep(interval);
  };

  for _ in 1..loops {
    run_then_sleep(&mut env, true);
    for _ in 1..6 {
      run_then_sleep(&mut env, false);
    }
  }
}
