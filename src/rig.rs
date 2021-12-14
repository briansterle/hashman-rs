use std::process::Command;

use sysinfo::ProcessExt;

use crate::gpu::{GPULoad, WindowsGPU, GPU};
use crate::mining::Mining;
use crate::{Config, Sys};

#[derive(Debug)]
pub enum Rig {
  Idle,
  Mining,
  Gaming,
}

impl Rig {
  pub fn state(sys: &Sys, gpu: &WindowsGPU) -> Rig {
    let load: GPULoad = gpu.get_util().expect("error getting gpu util");

    match sys.processes().find(|p| Mining::is_hash_binary(p.name())) {
      Some(_) if (load.is_hot()) => Rig::Mining,
      None if load.is_hot() => {
        println!("hot & gaming");
        Rig::Gaming
      }
      Some(_) => {
        // not hot, but mining
        Mining::kill_all(sys);
        Rig::Idle
      }
      None => {
        // not hot, not mining
        println!("system idle");
        Rig::Idle
      }
    }
  }

  pub fn move_state(self, config: &Config) -> Rig {
    if self == Rig::Idle {
      let mine = Command::new(&config.miner_exe);
      Mining::restart_async(mine).expect("oops")
    } else {
      self
    }
  }
}

impl PartialEq for Rig {
  fn eq(&self, other: &Self) -> bool {
    format!("{:?}", self) == format!("{:?}", other)
  }
}
