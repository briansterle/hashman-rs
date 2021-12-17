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
    let env = HashEnv {
      conf: config::json(),
      sys: Sys {
        system: System::new_all(),
      },
      gpu: GPU::new(&config::json().py_gputil, &config::json().py_exec),
    };
    println!("Hashman [INFO] env = {:#?}", env);
    env
  }

  pub fn run(&mut self) -> Rig {
    let current: Rig = Rig::state(self);
    println!("Hashman [INFO] Rig::state = {:?}", current);
    Rig::move_state(current, self)
  }
}

#[cfg(test)]
mod tests {
  use sysinfo::SystemExt;

  use crate::config::Config;
  use crate::rig::Rig;
  use crate::sys::Sys;
  use crate::{config, HashEnv};

  #[test]
  fn config_parses() {
    let config: Config = config::json();
    assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
    assert_eq!(config.gpu_p1, vec!["Notepad.exe"]);
    assert!(config.gpu_p2.contains(&"NiceHashMiner.exe".to_string()));
  }

  #[test]
  fn rig_gets_state() {
    let mut env = HashEnv::setup();
    let _state = Rig::state(&mut env);
    assert_eq!(_state, Rig::Mining);
  }

  #[test]
  fn gets_priority_processes() {
    let mut sys = Sys {
      system: sysinfo::System::new_all(),
    };
    let pids = &mut sys.fetch_pids();
    assert!(!pids.mining.is_empty());
  }

  #[test]
  fn run_debug() {
    let updated: Rig = HashEnv::setup().run();
    assert_eq!(updated, Rig::Mining)
  }
}
