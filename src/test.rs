#[cfg(test)]
mod tests {
  use sysinfo::{Process, ProcessExt, SystemExt};

  use crate::{config, Config, GPU, Rig, WindowsGPU};
  use crate::mining::Mining;
  use crate::sys::Sys;

  #[test]
  fn config_parses() {
    let config: Config = config::json();
    assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
    assert_eq!(config.gpu_primary, vec!["game.exe"]);
    assert_eq!(config.gpu_secondary, vec!["nicehash.exe"]);
  }

  #[test]
  fn rig_gets_state() {
    let sys = Sys {
      system: sysinfo::System::new_all(),
    };

    let conf: Config = config::json();
    let wgpu: WindowsGPU = GPU::new(conf.py_gputil, conf.py_exec);
    let _state = Rig::get_state(&sys, &wgpu);
  }

  #[test]
  fn nicehash_is_a_hash_binary() {
    let is = Mining::is_hash_binary("nicehash");
    assert!(is);
  }

  #[test]
  fn syts_gets_cargo_process() {
    let sys = Sys {
      system: sysinfo::System::new_all(),
    };

    fn filter_ps(p: &Process) -> bool {
      p.name().to_lowercase().contains("cargo")
    }

    // let ps = sys.processes_with(|p| p.name().to_lowercase().contains("cargo"));
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
