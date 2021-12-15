#![allow(dead_code)]

use sysinfo::{System, SystemExt};

use gpu::{WindowsGPU, GPU};

pub use crate::config::Config;
pub use crate::rig::Rig;
pub use crate::sys::Sys;

mod config;
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
    let current: Rig = Rig::state(&self);
    println!("Hashman [INFO] Rig::state = {:?}", current);
    current.move_state(&self)
  }
}

#[cfg(test)]
mod tests {
  use sysinfo::SystemExt;

  use crate::config::Config;
  use crate::gpu::{WindowsGPU, GPU};
  use crate::mining::Mining;
  use crate::rig::Rig;
  use crate::sys::Sys;
  use crate::{config, HashEnv};

  #[test]
  fn config_parses() {
    let config: Config = config::json();
    assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
    assert_eq!(config.gpu_p1, vec!["game.exe"]);
    assert_eq!(config.gpu_p2, vec!["NiceHashMiner.exe"]);
  }

  #[test]
  fn rig_gets_state() {
    let env = HashEnv::setup();
    let _state = Rig::state(&env);
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

  #[test]
  fn gets_priority_processes() {
    let sys = Sys {
      system: sysinfo::System::new_all(),
    };
    let (ps1, ps2) = sys.priority_processes(&config::json().gpu_p1, &config::json().gpu_p2);
    assert!(ps1.is_empty());
    assert!(!ps2.is_empty()); // must be mining to pass this
  }
}
