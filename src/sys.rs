use std::collections::hash_map::Values;
use std::process::Command;
use std::str;

use sysinfo::{Pid, Process, ProcessExt, SystemExt};

pub struct Sys {
  pub system: sysinfo::System,
}

impl Sys {
  pub fn tasks() -> Vec<String> {
    let output = Command::new("tasklist.exe").output().unwrap();
    let stdout = output.stdout;
    // let out = String::from_utf8_lossy(&stdout);
    let out = str::from_utf8(&stdout).unwrap();

    out.split("\n").map(str::to_string).collect()
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
}
