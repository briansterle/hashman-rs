use hashman_rs::{HashEnv, Rig};

fn main() {
  let updated: Rig = HashEnv::setup().run();
  println!("Hashman [INFO] Rig::move_state = {:?}", updated);
}
