use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

pub fn read() -> String {
    fs::read_to_string("config.json").expect("oops")
}
pub fn json() -> Config {
    let conf: Config = serde_json::from_str(&read())?;
    conf
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    mining_exe: String
    // python_exec: String,
    // python_gpu_util: String,
}