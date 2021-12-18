#![allow(dead_code)]

use std::collections::HashMap;
use std::env;

use sysinfo::{System, SystemExt};

use gpu::{WindowsGPU, GPU};

pub use crate::config::Config;
pub use crate::rig::Rig;
pub use crate::sys::Sys;

mod config;
mod gpu;
mod mining;
mod rig;
mod sys;

fn miner_path() -> String {
  env::var("HASHMAN_MINER_PATH").expect("please set HASHMAN_MINER_PATH env var")
}

fn gaming_path() -> String {
  env::var("HASHMAN_GAMING_PATH").expect("please set HASHMAN_GAMING_PATH env var")
}

#[derive(Debug)]
pub struct HashEnv {
  conf: HashPath,
  sys: Sys,
  gpu: WindowsGPU,
}

const GPUTIL_PY: &'static str = "import GPUtil; print(GPUtil.getGPUs().pop().load)";
const PYTHON: &'static str = "python";
// const HASH_CONF: &'static str = "~/.hashman";

fn hash_conf_dir() -> String {
  let home = dirs::home_dir()
    .expect("no $HOME dir found!")
    .to_str()
    .unwrap()
    .to_string();
  format!("{}\\.hashman", home)
}

type Exe = str;

pub fn hash_path() -> String {
  format!("{}\\hashpath.txt", hash_conf_dir())
}

#[derive(Debug)]
struct HashPath {
  mining_path: Vec<String>,
  gaming_path: Vec<String>,
  miner_exe: String,
}

impl HashPath {
  fn parse(str: &str) -> Result<Self, ()> {
    let lines = str.split("\n");
    let splitty: HashMap<&str, Vec<String>> = HashMap::from_iter(
      lines
        .filter(|line| !line.is_empty())
        .map(|line| {
          let kv: Vec<&str> = line.split("=").collect();
          return (kv[0], kv[1]);
        })
        .map(|(name, path)| {
          let paths: Vec<String> = path.split(",").map(|s| s.to_string()).collect();
          return (name, paths);
        }),
    );

    return Ok(HashPath {
      mining_path: splitty.get("mining_path").unwrap().to_vec(),
      gaming_path: splitty.get("gaming_path").unwrap().to_vec(),
      miner_exe: splitty
        .get("miner_exe")
        .unwrap()
        .first()
        .unwrap()
        .to_string(),
    });
  }

  pub fn fetch() -> Result<Self, ()> {
    // if ~/.hashman does not exist create it
    // if  ~/.hashman/hashpath.txt does not exist create/load defaults
    // then parse dirs
    if std::path::Path::new(&hash_conf_dir()).is_dir() {
      let hp = &hash_path();
      println!("checking hash path @ {}", hp);
      if std::path::Path::new(hp).is_file() {
        // parse dat
        let data = &std::fs::read_to_string(hash_path()).unwrap();
        return HashPath::parse(data);
      } else {
        println!("HASH_PATH not a file")
      }
    } else {
      println!("HASH_CONF not a dir")
    }
    panic!("couldn't load the HashPath");
    // Ok(HashPath {
    //   mining_path: vec![],
    //   gaming_path: vec![],
    // })
  }
}

impl HashEnv {
  pub fn setup() -> Self {
    let env = HashEnv {
      conf: HashPath::fetch().expect("couldn't parse the HASH_PATH"),
      sys: Sys {
        system: System::new_all(),
      },
      gpu: GPU::new(PYTHON, GPUTIL_PY),
    };
    println!("Hashman [INFO] env = {:#?}", env);
    env
  }

  pub fn run(&mut self) -> Rig {
    let current: Rig = Rig::state(self);
    println!("Hashman [INFO] Rig::state = {:?}", current);
    Rig::move_state(current, self)
  }
}

#[cfg(test)]
mod tests {
  use sysinfo::SystemExt;

  use crate::config::Config;
  use crate::rig::Rig;
  use crate::sys::Sys;
  use crate::{config, HashEnv, HashPath};

  #[test]
  fn hashpath_fetch() {
    let res = HashPath::fetch();
    assert!(res.is_ok());
    let hp = res.unwrap();
  }

  #[test]
  fn hashpath_parse() {
    let contents = r#"
gaming_path=Notepad.exe,D:\GAMES\steamapps\common
mining_path=NiceHashMiner.exe,app_nhm.exe
miner_exe=C:\Users\brian\AppData\Local\Programs\NiceHash Miner\NiceHashMiner.exe
"#;
    let res = HashPath::parse(contents);
    assert!(res.is_ok());
    let hp = res.unwrap();
    assert_eq!(hp.mining_path, vec!["NiceHashMiner.exe", "app_nhm.exe"]);
    assert_eq!(
      hp.gaming_path,
      vec!["Notepad.exe", "D:\\GAMES\\steamapps\\common"]
    );
    assert_eq!(
      hp.miner_exe,
      "C:\\Users\\brian\\AppData\\Local\\Programs\\NiceHash Miner\\NiceHashMiner.exe"
    )
  }

  #[test]
  fn config_parses() {
    let config: Config = config::json();
    assert!(config.miner_exe.ends_with("NiceHashMiner.exe"));
    assert!(config.gpu_p1.contains(&"Notepad.exe".to_string()));
    assert!(config.gpu_p2.contains(&"NiceHashMiner.exe".to_string()));
  }

  #[test]
  fn rig_gets_state() {
    let mut env = HashEnv::setup();
    let _state = Rig::state(&mut env);
    assert_eq!(_state, Rig::Mining);
  }

  #[test]
  fn gets_priority_processes() {
    let mut sys = Sys {
      system: sysinfo::System::new_all(),
    };
    let pids = &mut sys.fetch_pids();
    assert!(!pids.mining.is_empty());
  }

  #[test]
  fn run_debug() {
    let updated: Rig = HashEnv::setup().run();
    assert_eq!(updated, Rig::Mining)
  }
}
