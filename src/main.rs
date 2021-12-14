#![allow(dead_code)]

use sysinfo::{System, SystemExt};

use hashman_rs::{run, HashEnv};

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
  let env = HashEnv::setup();
  let updated = run(env);
  println!("Hashman [INFO] rig::move_state {:?}", updated);
}
