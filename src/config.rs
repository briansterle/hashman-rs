use std::fs;

use serde::{Deserialize, Serialize};

const JSON_CONF_FILE: &str  = if cfg!(windows) {
  "config.json"
} else {
  "config-linux.json"
};

#[inline(always)]
pub fn json() -> Config {
  serde_json::from_str(
    &fs::read_to_string(JSON_CONF_FILE).unwrap()
  ).expect("config.json is invalid")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub miner_exe: String,
  pub py_exec: String,
  pub py_gputil: String,
  pub gpu_p1: Vec<String>,
  pub gpu_p2: Vec<String>,
}
