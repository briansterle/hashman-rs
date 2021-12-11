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
    pub py_exec: String,
    pub py_gputil: String,
}
