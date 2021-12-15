use std::collections::HashSet;
use std::process::Command;
use std::{thread as std_thread, time};

use crossbeam::thread;
use sysinfo::{ProcessExt, Signal};

use crate::rig::Rig;
use crate::{HashEnv, Sys};

pub struct Mining {}

// todo get from &Config
fn get_hash_bins() -> HashSet<&'static str> {
  HashSet::from(["app_nhm", "nicehash", "nice"])
}

impl Mining {
  pub fn restart_async(mine: Command) -> Result<Rig, ()> {
    thread::scope(|scope| {
      scope.spawn(move |_| Mining::restart(mine));
    })
    .unwrap();
    Ok(Rig::Mining)
  }

  fn restart(mut mine: Command) -> Result<Rig, Rig> {
    let output = mine.output().expect("failed to start mining process");
    match output.status.code() {
      Some(code) if code == 0 => Ok(Rig::Mining),
      Some(code) => {
        println!("Unexpectedly exited mining exe with status code: {}", code);
        Err(Rig::Idle)
      }
      None => Err(Rig::Idle),
    }
  }

  pub fn kill_all(sys: &Sys, gpu_p2: &Vec<String>) -> bool {
    println!("Killing all mining processes...");
    let mut killed = false;

    let (_, ps2) = sys.priority_processes(&vec![], gpu_p2);
    ps2.into_iter().for_each(|p| {
      while !killed {
        killed |= p.kill(Signal::Kill);
        std_thread::sleep(time::Duration::from_millis(420));
      }
    });
    killed
  }
}
