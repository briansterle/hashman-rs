use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::process::Command;
use std::str;

use sysinfo::{Pid, Process, ProcessExt, SystemExt};

#[derive(Debug)]
pub struct Sys {
  pub system: sysinfo::System,
}

impl Sys {
  pub fn tasks() -> Vec<String> {
    let output = Command::new("tasklist.exe").output().unwrap();
    let stdout = output.stdout;
    let out = str::from_utf8(&stdout).unwrap();
    out.split('\n').map(str::to_string).collect()
  }

  pub fn processes(&self) -> Values<Pid, Process> {
    self.system.processes().values()
  }

  pub fn processes_matching(&self, needle: &str) -> Vec<&Process> {
    self
      .processes()
      .filter(|p| p.name().to_lowercase().contains(needle))
      .collect()
  }

  pub fn priority_processes(
    &self,
    gp1s: Vec<String>,
    gp2s: Vec<String>,
  ) -> HashMap<u8, Vec<&Process>> {
    let mut priority_ps = HashMap::from([(1, vec![]), (2, vec![])]);

    for p in self.processes() {
      if gp1s.contains(&p.name().to_owned()) {
        priority_ps.get_mut(&1).unwrap().append(&mut vec![p]);
      } else if gp2s.contains(&p.name().to_owned()) {
        priority_ps.get_mut(&2).unwrap().append(&mut vec![p]);
      }
    }
    priority_ps
  }
}
