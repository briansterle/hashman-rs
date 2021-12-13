#[cfg(test)]
mod tests {
  use crate::mining::Mining;
  use crate::{config, Config, Rig, WindowsGPU, GPU};

  #[test]
  fn parses_config() {
    let config: Config = config::json();
    assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
  }

  // #[test]
  // fn mining_kills_all() {
  //     Mining::kill_all();
  // }

  #[test]
  fn gets_tasks() {
    let tasks = Rig::tasks();
    assert!(!&tasks.is_empty());

    let tasks_ref = &mut tasks.into_iter();
    assert!(tasks_ref.any(|s| s.contains("cargo")));
  }

  #[test]
  fn rig_gets_state() {
    let conf: Config = config::json();
    let wgpu: WindowsGPU = GPU::new(conf.py_gputil, conf.py_exec);
    let _state = Rig::get_state(&wgpu);
  }

  // #[test]
  // fn app_works() {
  //     main();
  // }
}
