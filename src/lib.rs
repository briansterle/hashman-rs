use sysinfo::{System, SystemExt};

use gpu::{GPULoad, WindowsGPU, GPU};
use mining::Mining;

use crate::config::Config;
use crate::rig::Rig;
use crate::sys::Sys;

mod config;
mod gaming;
mod gpu;
mod mining;
mod rig;
mod sys;
mod test;

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
}

pub fn run(env: HashEnv) -> Rig {
  let current: Rig = Rig::state(&env.sys, &env.gpu);
  println!("Hashman [INFO] Rig::get_state {:?}", current);
  current.move_state(&env.conf)
}
