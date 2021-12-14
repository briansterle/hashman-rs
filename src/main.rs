use hashman_rs::{HashEnv, Rig};

fn main() {
  let env: HashEnv = HashEnv::setup();
  println!("Hashman [INFO] env = {:#?}", env);
  let updated: Rig = env.run();
  println!("Hashman [INFO] rig::move_state = {:?}", updated);
}
