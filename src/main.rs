use std::env;
use std::time::Duration;

use hashman_rs::{HashEnv, Rig};

fn main() {
  let args = env::args().collect::<Vec<String>>();
  let loops: u64 = match args.get(1) {
    None => 1,
    Some(count) => count.parse::<u64>().unwrap(),
  };

  let refresh_interval = match args.get(2) {
    None => 1,
    Some(count) => count.parse::<u64>().unwrap(),
  };

  let mut i = 0;
  while i < loops {
    i += 1;
    let updated: Rig = HashEnv::setup().run();
    println!("Hashman [INFO] Rig::move_state = {:?}", updated);
    std::thread::sleep(Duration::from_secs(refresh_interval));
  }
}
