use std::collections::HashSet;
use std::process::Command;
use std::{thread as std_thread, time};

use crossbeam::thread;
use sysinfo::{ProcessExt, Signal, SystemExt};

use crate::rig::Rig;
use crate::Config;

pub struct Mining;

fn get_hash_bins() -> HashSet<&'static str> {
  HashSet::from(["app_nhm", "nicehash", "hash", "nice"])
}

impl Mining {
  pub fn restart_async(config: &Config) -> Result<Rig, ()> {
    thread::scope(|scope| {
      scope.spawn(move |_| Mining::restart(config));
    })
    .unwrap();
    Ok(Rig::Mining(false))
  }

  fn restart(config: &Config) -> Result<Rig, Rig> {
    let output = Command::new(&config.miner_exe)
      .output()
      .expect("failed to start mining process");
    match output.status.code() {
      Some(code) if code == 0 => Ok(Rig::Mining(false)),
      Some(code) => {
        println!("Unexpectedly exited mining exe with status code: {}", code);
        Err(Rig::Idle(false))
      }
      None => Err(Rig::Idle(false)),
    }
  }

  pub fn is_hash_binary(proc_name: &str) -> bool {
    get_hash_bins()
      .iter()
      .any(|bin| proc_name.to_lowercase().contains(bin))
  }

  pub fn kill_all() -> bool {
    let system = sysinfo::System::new_all();
    let ps = system.processes().values();
    let mut killed = false;
    ps.filter(|p| Mining::is_hash_binary(p.name()))
      .for_each(|p| {
        while !killed {
          killed |= p.kill(Signal::Kill);
          std_thread::sleep(time::Duration::from_millis(420));
        }
      });
    killed
  }
}
