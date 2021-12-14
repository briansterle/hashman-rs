#![allow(dead_code)]

use sysinfo::{System, SystemExt};

use gpu::{WindowsGPU, GPU};

pub use crate::config::Config;
pub use crate::rig::Rig;
pub use crate::sys::Sys;

mod config;
mod gaming;
mod gpu;
mod mining;
mod rig;
mod sys;

#[derive(Debug)]
pub struct HashEnv {
  conf: Config,
  sys: Sys,
  gpu: WindowsGPU,
}

impl HashEnv {
  pub fn setup() -> Self {
    HashEnv {
      conf: config::json(),
      sys: Sys {
        system: System::new_all(),
      },
      gpu: GPU::new(&config::json().py_gputil, &config::json().py_exec),
    }
  }

  pub fn run(&self) -> Rig {
    let current: Rig = Rig::state(&self.sys, &self.gpu);
    println!("Hashman [INFO] Rig::state {:?}", current);
    current.move_state(&self.conf)
  }
}

#[cfg(test)]
mod tests {
  use sysinfo::SystemExt;

  use crate::config;
  use crate::config::Config;
  use crate::gpu::{WindowsGPU, GPU};
  use crate::mining::Mining;
  use crate::rig::Rig;
  use crate::sys::Sys;

  #[test]
  fn config_parses() {
    let config: Config = config::json();
    assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
    assert_eq!(config.gpu_p1, vec!["game.exe"]);
    assert_eq!(config.gpu_p2, vec!["nicehash.exe"]);
  }

  #[test]
  fn rig_gets_state() {
    let sys = Sys {
      system: sysinfo::System::new_all(),
    };

    let conf: Config = config::json();
    let wgpu: WindowsGPU = GPU::new(&conf.py_gputil, &conf.py_exec);
    let _state = Rig::state(&sys, &wgpu);
  }

  #[test]
  fn nicehash_is_a_hash_binary() {
    let is = Mining::is_hash_binary("nicehash");
    assert!(is);
  }

  #[test]
  fn sys_gets_cargo_process() {
    let sys = Sys {
      system: sysinfo::System::new_all(),
    };

    let ps = sys.processes_matching("cargo");
    assert!(!ps.is_empty());
  }

  #[test]
  fn sys_gets_tasks() {
    let tasks = Sys::tasks();
    assert!(!&tasks.is_empty());

    let tasks_ref = &mut tasks.into_iter();
    assert!(tasks_ref.any(|s| s.contains("cargo")));
  }
}
