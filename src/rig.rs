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
  fn or_idle(self, load: &GPULoad) -> Self {
    if load.is_hot() {
      self
    } else {
      Self::Idle
    }
  }

  fn on_idle<F>(self, load: &GPULoad, on_idle: F) -> Self
  where
    F: FnOnce() -> bool,
  {
    if load.is_hot() {
      self
    } else {
      on_idle();
      Self::Idle
    }
  }

  pub fn state(env: &HashEnv) -> Self {
    let load: GPULoad = env.gpu.get_util().expect("error getting gpu util");
    let (gaming_ps, mining_ps) = env
      .sys
      .priority_processes(&env.conf.gpu_p1, &env.conf.gpu_p2);
    match (gaming_ps.is_empty(), mining_ps.is_empty()) {
      (false, false) => Self::Conflict,
      (false, true) => Self::Gaming.or_idle(&load),
      (true, false) => Self::Mining.on_idle(&load, || Mining::kill_all(&env.sys, &env.conf.gpu_p2)),
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
