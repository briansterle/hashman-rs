use std::process::Command;

use sysinfo::{Pid, Process, ProcessExt};

use crate::gpu::{GPULoad, WindowsGPU, GPU};
use crate::mining::Mining;
use crate::{Config, HashEnv, Sys};

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

    fn map_pid(ps: Vec<&Process>) -> Vec<Pid> {
      ps.into_iter().map(|p| p.pid()).collect()
    }

    let (gaming_ps, mining_ps) = env
      .sys
      .priority_processes(&env.conf.gpu_p1, &env.conf.gpu_p2);
    match (gaming_ps.is_empty(), mining_ps.is_empty()) {
      (false, false) => Self::Conflict {
        gaming: map_pid(gaming_ps),
        mining: map_pid(mining_ps),
      },
      (false, true) => Self::Gaming.or_idle(&load),
      (true, false) => Self::Mining.on_idle(&load, || Mining::kill_all(&env.sys, &env.conf.gpu_p2)),
      (true, true) => Self::Idle,
    }
  }

  pub fn move_state(self, env: &HashEnv) -> Self {
    match self {
      Self::Idle => Mining::restart_async(Command::new(&env.conf.miner_exe))
        .expect("failed to restart miner.exe"),
      Self::Mining => self,
      Self::Gaming => self,
      Self::Conflict { gaming, mining } => {
        let mining_processes = mining
          .into_iter()
          .map(|pid| env.sys.lookup(pid))
          .filter(|o| o.is_some())
          .map(|o| o.unwrap())
          .collect();
        Mining::kill_processes(mining_processes);
        Self::Gaming
      }
    }
  }
}
