use std::process::Command;

use sysinfo::ProcessExt;

use crate::gpu::{GPULoad, WindowsGPU, GPU};
use crate::mining::Mining;
use crate::{Config, HashEnv, Sys};

#[derive(Debug, PartialEq)]
pub enum Rig {
  Idle,
  Mining,
  Gaming,
  Conflict,
}

impl Rig {
  fn or_idle(rig_state: Self, load: &GPULoad) -> Self {
    if load.is_hot() {
      rig_state
    } else {
      Self::Idle
    }
  }

  pub fn state(env: &HashEnv) -> Self {
    let load: GPULoad = env.gpu.get_util().expect("error getting gpu util");
    let (ps1, ps2) = env
      .sys
      .priority_processes(&env.conf.gpu_p1, &env.conf.gpu_p2);
    match (ps1.is_empty(), ps2.is_empty()) {
      (false, false) => Self::Conflict,
      (false, true) => Self::Gaming,
      (true, false) => {
        if load.is_hot() {
          Self::Mining
        } else {
          // not hot, but mining
          Mining::kill_all(&env.sys);
          Self::Idle
        }
      }
      (true, true) => Self::Idle,
    }
  }

  pub fn move_state(self, config: &Config) -> Self {
    match self {
      Self::Idle => {
        let mine = Command::new(&config.miner_exe);
        Mining::restart_async(mine).expect("oops")
      }
      Self::Mining => self,
      Self::Gaming => self,
      Self::Conflict => self,
    }
  }
}
