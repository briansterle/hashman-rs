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
    thread::spawn(move || Mining::run_restart_cmd(miner).expect("miner.exe crashed"));
    thread::sleep(Duration::from_millis(42));
    Rig::Mining
  }

  fn run_restart_cmd(mut mine: Command) -> Result<Rig, Rig> {
    let output = mine.output().expect("failed to start mining process");
    match output.status.code() {
      Some(code) if code == 0 => Ok(Rig::Mining),
      Some(code) => {
        debug!("Unexpectedly exited mining exe with status code: {}", code);
        Err(Rig::Idle)
      }
      None => Err(Rig::Idle),
    }
  }

  pub fn kill(env: &mut HashEnv) {
    let kill_pids = env.sys.fetch_pids(&env.hash_path).mining;
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
