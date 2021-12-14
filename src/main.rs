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

fn main() {
  let sys = Sys {
    system: System::new_all(),
  };

  println!("Hashman [INFO] Reading config...");
  let conf: Config = config::json();

  println!("Hashman [INFO] config: {:?}", conf);
  let wgpu: WindowsGPU = GPU::new(conf.py_gputil.clone(), conf.py_exec.clone());

  let current: Rig = Rig::get_state(&sys, &wgpu);
  println!("Hashman [INFO] Rig::get_state {:?}", current);

  let updated: Rig = current.move_state(&conf);
  println!("Hashman [INFO] rig::move_state {:?}", updated);
}
