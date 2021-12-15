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

  #[inline(always)]
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
    gp1s: &Vec<String>,
    gp2s: &Vec<String>,
  ) -> (Vec<&Process>, Vec<&Process>) {
    let mut p1 = vec![];
    let mut p2 = vec![];

    for p in self.processes() {
      if gp1s.contains(&p.name().to_owned()) {
        println!("{}", Self::pretty_proc(p, "p1 gaming"));
        p1.push(p);
      } else if gp2s.contains(&p.name().to_owned()) {
        println!("{}", Self::pretty_proc(p, "p2 mining"));
        p2.push(p);
      }
    }
    (p1, p2)
  }

  #[inline(always)]
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
