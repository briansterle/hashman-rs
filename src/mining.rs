use std::io::Error;
use std::process::Command;
use std::time::Duration;
use std::{thread, time};

use log::debug;
use sysinfo::{ProcessExt, Signal};

use crate::rig::Rig;
use crate::{Exe, HashEnv};

pub struct Mining {}

impl Mining {
  pub fn restart(miner_exe: &Exe) -> Rig {
    let miner = Command::new(&miner_exe);
    thread::spawn(move || Mining::run_restart_cmd(miner).unwrap_or(Rig::Idle));
    thread::sleep(Duration::from_millis(42));
    Rig::Mining
  }

  fn run_restart_cmd(mut mine: Command) -> std::io::Result<Rig> {
    let output = mine.output()?;

    if let Some(code) = output.status.code() {
      if code == 0 {
        Ok(Rig::Mining)
      } else {
        Ok(Rig::Idle)
      }
    } else {
      Ok(Rig::Idle)
    }
  }

  pub fn kill(env: &mut HashEnv) {
    let kill_pids = env.sys.refresh_pids(&env.hash_path).mining;
    for pid in kill_pids {
      loop {
        match env.sys.lookup(pid) {
          None => break,
          Some(p) => p.kill(Signal::Kill),
        };
        thread::sleep(time::Duration::from_millis(42));
      }
    }
  }
}
