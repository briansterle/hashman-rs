use hashman_rs::{run, HashEnv};

fn main() {
  let env: HashEnv = HashEnv::setup();
  let updated = run(env);
  println!("Hashman [INFO] rig::move_state {:?}", updated);
}
