use core::time;
use std::thread;

use log::debug;

use crate::gpu::Gpu;
use crate::mining::Mining;
use crate::HashEnv;

#[derive(Debug, PartialEq)]
pub enum Rig {
  Idle,
  Mining,
  Gaming,
  Conflict,
}

impl Rig {
  fn if_idle(self, env: &mut HashEnv) -> Self {
    let max_tries = 90;
    let mut tries = 0;

    while tries < max_tries {
      if env.gpu.is_hot() {
        return self;
      } else {
        debug!("Sleeping until gpu_load.is_hot...");
        thread::sleep(time::Duration::from_millis(1000));
      }
      tries += 1;
    }
    Mining::kill(env);
    Self::Idle
  }

  pub fn state(env: &mut HashEnv) -> Self {
    let hp = &env.hash_path;
    let pids = env.sys.refresh_pids(hp);
    match (pids.gaming.is_empty(), pids.mining.is_empty()) {
      (true, true) => Self::Idle,
      (false, true) => Self::Gaming,
      (true, false) => Self::Mining.if_idle(env),
      (false, false) => Self::Conflict,
    }
  }

  pub fn move_state(current: Rig, env: &mut HashEnv) -> Self {
    match current {
      Self::Idle => Mining::restart(&env.hash_path.miner_exe),
      Self::Mining => current,
      Self::Gaming => current,
      Self::Conflict => {
        Mining::kill(env);
        let mut mining_pids = env.sys.refresh_pids(&env.hash_path).mining;
        while !mining_pids.is_empty() {
          debug!("mining_pids still live: {:?}", mining_pids);
          Mining::kill(env);
          mining_pids = env.sys.refresh_pids(&env.hash_path).mining;
        }
        Self::Gaming
      }
    }
  }
}
