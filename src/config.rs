use std::fs;

use serde::{Deserialize, Serialize};

pub fn read() -> String {
    fs::read_to_string("config.json").expect("oops")
}
pub fn json() -> Config {
    let conf = serde_json::from_str(&read()).expect("oops");
    conf
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mining_exe: String
    // python_exec: String,
    // python_gpu_util: String,
}