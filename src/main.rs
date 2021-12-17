use std::env;
use std::time::Duration;

use hashman_rs::{HashEnv, Rig};

fn main() {
  let loops: u64 = match env::args().collect().get(1) {
    None => 1,
    Some(count) => count.parse::<u64>().unwrap(),
  };
  let mut i = 0;
  while i < loops {
    i += 1;
    let updated: Rig = HashEnv::setup().run();
    println!("Hashman [INFO] Rig::move_state = {:?}", updated);
    std::thread::sleep(Duration::from_secs(1));
  }
}
