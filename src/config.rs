use std::fs;

use serde::{Deserialize, Serialize};

pub fn read() -> String {
  fs::read_to_string("config.json").expect("config.json is invalid")
}

pub fn json() -> Config {
  serde_json::from_str(&read()).expect("config.json is invalid")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub miner_exe: String,
  pub py_exec: String,
  pub py_gputil: String,
  pub gpu_p1: Vec<String>,
  pub gpu_p2: Vec<String>,
}
