use std::time::Duration;

use log::info;

use hashman_rs::{HashEnv, Rig};

fn main() {
  env_logger::init();

  let args = std::env::args().collect::<Vec<String>>();
  let loops: u64 = match args.get(1) {
    None => 1,
    Some(count) => count.parse().unwrap(),
  };

  let refresh_interval: u64 = match args.get(2) {
    None => 1,
    Some(count) => count.parse().unwrap(),
  };

  let mut i = 0;
  while i < loops {
    i += 1;
    let updated: Rig = HashEnv::setup().run();
    info!("Rig::move_state = {:?}", updated);
    std::thread::sleep(Duration::from_secs(refresh_interval));
  }
}
