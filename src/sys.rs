use std::{fmt, str};

use log::{debug, info};
use sysinfo::{Pid, Process, ProcessExt, SystemExt};

use crate::HashPath;

#[derive(Debug)]
pub struct Sys {
  pub system: sysinfo::System,
  pub pids: Pids,
}

#[derive(Debug, Clone)]
pub struct Pids {
  pub gaming: Vec<Pid>,
  pub mining: Vec<Pid>,
}

// #[derive(Debug, Clone)]
// pub struct PrettyProc {
//   pp: String,
// }
#[derive(Debug, Clone)]
struct Proc {
  pid: Pid,
  parent: Pid,
  name: String,
}

#[derive(Clone)]
pub struct PrettyProcs(Vec<Proc>);

impl fmt::Debug for PrettyProcs {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    if !self.0.is_empty() {
      fmt.write_str("{\n")?;
      for proc in &self.0 {
        fmt.write_str(format!("\t{:?}\n", proc).as_str())?;
      }
      fmt.write_str("\n}")
    } else {
      fmt.write_str("{}")
    }
  }
}

impl Pids {
  pub fn into_process(self, sys: &mut Sys) -> (PrettyProcs, PrettyProcs) {
    let mut do_map = |pids: Vec<Pid>| -> PrettyProcs {
      let mut pps: Vec<Proc> = vec![];

      for pid in pids.into_iter() {
        if let Some(pp) = sys.lookup(pid).map(|p| Proc {
          pid,
          parent: p.parent().unwrap_or(pid),
          name: p.name().to_string(),
        }) {
          pps.push(pp);
        }
      }
      PrettyProcs(pps)
    };
    (do_map(self.gaming), do_map(self.mining))
  }

  pub const DEFAULT: Self = Pids {
    gaming: vec![],
    mining: vec![],
  };
  pub fn is_empty(&self) -> bool {
    self.gaming.is_empty() && self.mining.is_empty()
  }
}

impl Sys {
  fn refresh(&mut self) -> &mut Self {
    self.system.refresh_processes();
    self
  }

  pub fn refresh_pids(&mut self, hash_path: &HashPath) -> Pids {
    let _ = &self.refresh().system;

    if self.pids.is_empty() {
      return self.fetch_pids(hash_path);
    } else {
      let mut no_fetch = false;

      for pid in &self.pids.mining {
        debug!("Refreshing mining process: {:#?}", pid);
        no_fetch |= self.system.refresh_process(*pid);
      }

      for pid in &self.pids.gaming {
        debug!("Refreshing gaming process: {:#?}", pid);
        no_fetch |= self.system.refresh_process(*pid);
      }

      if !no_fetch {
        self.fetch_pids(hash_path);
      }
    }
    self.pids.to_owned()
  }

  pub fn fetch_pids(&mut self, hash_path: &HashPath) -> Pids {
    info!("Running a deep process fetch...");
    let mut p1: Vec<Pid> = vec![];
    let mut p2: Vec<Pid> = vec![];
    let system = &self.refresh().system;
    let processes = system.processes();

    // initial search for gaming and mining parent processes
    for (pid, p) in processes {
      if hash_path.gaming_path.contains(&p.name().to_string()) {
        //  debug!("{}", Self::pretty_proc(p, "Gaming Process"));
        p1.push(*pid);
      } else if hash_path.mining_path.contains(&p.name().to_string()) {
        // debug!("{}", Self::pretty_proc(p, "Mining Process"));
        p2.push(*pid);
      }
    }

    // second search for the children of these parents
    for (pid, p) in self.refresh().system.processes() {
      if let Some(parent) = p.parent() {
        if p1.contains(&parent) && !p1.contains(pid) {
          // debug!("Gaming child: {}", pid);
          p1.push(*pid);
        } else if p2.contains(&parent) && !p2.contains(pid) {
          // debug!("Mining child: {}", pid);
          p2.push(*pid);
        }
      }
    }

    let pids = Pids {
      gaming: p1.to_vec(),
      mining: p2.to_vec(),
    };
    self.pids = pids.clone();

    info!("|Watched Processes| => {:#?}", pids.into_process(self));

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
