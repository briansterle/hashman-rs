use std::fs;

use serde::{Deserialize, Serialize};

#[inline(always)]
pub fn json() -> Config {
  let file = if cfg!(windows) {
    "config.json"
  } else {
    "config-linux.json"
  };
  let json_str = fs::read_to_string(file).unwrap();
  serde_json::from_str(&json_str).expect("config.json is invalid")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub miner_exe: String,
  pub py_exec: String,
  pub py_gputil: String,
  pub gpu_p1: Vec<String>,
  pub gpu_p2: Vec<String>,
}
