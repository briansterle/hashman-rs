#![allow(dead_code)]

use sysinfo::{System, SystemExt};

use crate::config::Config;
use crate::gpu::{WindowsGPU, GPU};
use crate::rig::Rig;
use crate::sys::Sys;

mod config;
mod gaming;
mod gpu;
mod mining;
mod rig;
mod sys;
mod test;

fn run(conf: &Config, sys: &Sys, gpu: &WindowsGPU) -> Rig {
  let current: Rig = Rig::state(sys, gpu);
  println!("Hashman [INFO] Rig::get_state {:?}", current);

  current.move_state(&conf)
}

fn main() {
  let sys = Sys {
    system: System::new_all(),
  };
  let conf: Config = config::json();

  println!("Hashman [INFO] Read config: {:#?}", conf);
  let gpu: WindowsGPU = GPU::new(conf.py_gputil.clone(), conf.py_exec.clone());

  let updated = run(&conf, &sys, &gpu);
  println!("Hashman [INFO] rig::move_state {:?}", updated);
}
