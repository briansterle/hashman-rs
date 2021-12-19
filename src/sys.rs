use std::str;

use log::debug;
use sysinfo::{Pid, Process, ProcessExt, SystemExt};

use crate::HashPath;

#[derive(Debug)]
pub struct Sys {
  pub system: sysinfo::System,
}

pub struct Pids {
  pub gaming: Vec<Pid>,
  pub mining: Vec<Pid>,
}

impl Sys {
  fn refresh(&mut self) -> &mut Self {
    self.system.refresh_processes();
    self
  }

  pub fn fetch_pids(&mut self, hash_path: &HashPath) -> Pids {
    let mut p1: Vec<Pid> = vec![];
    let mut p2: Vec<Pid> = vec![];
    let system = &self.refresh().system;
    let processes = system.processes();

    // initial search for gaming and mining parent processes
    for (pid, p) in processes {
      if hash_path.gaming_path.contains(&p.name().to_owned()) {
        debug!("{}", Self::pretty_proc(p, "Gaming Process"));
        p1.push(pid.to_owned());
      } else if hash_path.mining_path.contains(&p.name().to_owned()) {
        debug!("{}", Self::pretty_proc(p, "Mining Process"));
        p2.push(pid.to_owned());
      }
    }

    // second search for the children of these parents
    for (pid, p) in self.refresh().system.processes() {
      if let Some(parent) = p.parent() {
        if p1.contains(&parent) {
          debug!("Gaming child: {}", pid);
          p1.push(pid.to_owned());
        } else if p2.contains(&parent) {
          debug!("Mining child: {}", pid);
          p2.push(pid.to_owned());
        }
      }
    }

    Pids {
      gaming: p1,
      mining: p2,
    }
  }

  pub fn lookup(&mut self, pid: Pid) -> Option<&Process> {
    self.refresh().system.process(pid)
  }

  pub fn pretty_proc(p: &Process, p_type: &str) -> String {
    format!(
      "Found {} process [ \n\tname: {:#?} \n\tpid: {:?} \n\tparent: {:?} \n\tcmd: {:?} \n\tcpu_usage: {:#?} \n\tstatus: {:#?}\n]",
      p_type,
      p.name(),
      p.pid(),
      p.parent(),
      p.cmd(),
      p.cpu_usage(),
      p.status()
    )
  }
}
