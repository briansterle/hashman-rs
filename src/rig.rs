use std::process::Command;

use sysinfo::{Pid, ProcessExt};

use crate::gpu::{GPULoad, GPU};
use crate::mining::Mining;
use crate::{HashEnv, Sys};

#[derive(Debug, PartialEq)]
pub enum Rig {
  Idle,
  Mining,
  Gaming,
  Conflict { gaming: Vec<Pid>, mining: Vec<Pid> },
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
    F: FnOnce() -> (),
  {
    if load.is_hot() {
      self
    } else {
      on_idle();
      Self::Idle
    }
  }

  pub fn state(env: &mut HashEnv) -> Self {
    let load: GPULoad = env.gpu.get_util().expect("error getting gpu util");

    let (gaming_ps, mining_ps) = env.sys.priority_processes();

    match (gaming_ps.is_empty(), mining_ps.is_empty()) {
      (false, false) => Self::Conflict {
        gaming: Sys::to_pids(gaming_ps),
        mining: Sys::to_pids(mining_ps),
      },
      (false, true) => Self::Gaming.or_idle(&load),
      (true, false) => Self::Mining.on_idle(&load, || Mining::kill_processes(&mut env.sys, vec![])),
      (true, true) => Self::Idle,
    }
  }

  pub fn move_state(current: Rig, env: &mut HashEnv) -> Self {
    match current {
      Self::Idle => Mining::restart_async(Command::new(&env.conf.miner_exe))
        .expect("failed to restart miner.exe"),
      Self::Mining => current,
      Self::Gaming => current,
      Self::Conflict { gaming: _, mining } => {
        Mining::kill_processes(&mut env.sys, mining);
        Self::Gaming
      }
    }
  }
}
