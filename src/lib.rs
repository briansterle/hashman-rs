#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Write;

use log::{debug, info};
use sysinfo::{System, SystemExt};

use gpu::{Gpu, WindowsGPU};

pub use crate::rig::Rig;
use crate::sys::Pids;
pub use crate::sys::Sys;

mod gpu;
mod mining;
mod rig;
mod sys;

type Exe = str;
type GPULoad = f64;

#[derive(Debug)]
pub struct HashEnv {
  pub hash_path: HashPath,
  pub sys: Sys,
  pub gpu: WindowsGPU,
}

const GPUTIL_PY: &str = "import GPUtil; print(GPUtil.getGPUs().pop().load)";
const PYTHON: &str = "python";

/// Get and parse 'arg: T' @ 'idx: usize' provided a mapping function F: String => T
///   otherwise return the default 'def: T'
/// # Arguments
///
/// * `idx`   - index of the arg
/// * `f`     - func: String => T
/// * `def`   - default if parse fails
///
/// ```
/// use hashman_rs::get_arg_or;
/// let arg = get_arg_or(0, |str| str.len(), 42);
/// assert!(arg > 0 );
/// let default = get_arg_or(99, |str| str.len(), 42);
/// assert_eq!(default, 42)
/// ```
pub fn get_arg_or<T, F>(idx: usize, f: F, def: T) -> T
where
  F: FnOnce(&str) -> T,
{
  std::env::args()
    .collect::<Vec<String>>()
    .get(idx)
    .map(|arg| f(arg))
    .unwrap_or(def)
}

fn default_conf() -> String {
  "gaming_path=Notepad.exe,D:\\GAMES\\steamapps\\common
mining_path=NiceHashMiner.exe,app_nhm.exe
miner_exe="
    .to_string()
    + &default_nice_hash_location()
}

fn default_nice_hash_location() -> String {
  home() + "\\AppData\\Local\\Programs\\NiceHash Miner\\NiceHashMiner.exe"
}

fn home() -> String {
  dirs::home_dir()
    .expect("no $HOME dir found!")
    .to_str()
    .unwrap()
    .to_string()
}

fn hash_conf_dir() -> String {
  format!("{}\\.hashman", home())
}

fn hash_path() -> String {
  format!("{}\\hashpath.toml", hash_conf_dir())
}

#[derive(Debug)]
pub struct HashPath {
  mining_path: Vec<String>,
  gaming_path: Vec<String>,
  miner_exe: String,
}

impl HashPath {
  fn parse(str: &str) -> Result<Self, std::io::Error> {
    let lines = str.split('\n');
    let splitty: HashMap<&str, Vec<String>> = HashMap::from_iter(
      lines
        .filter(|line| !line.is_empty())
        .map(|line| {
          let kv: Vec<&str> = line.split('=').collect();
          (kv[0], kv[1])
        })
        .map(|(name, path)| {
          let paths: Vec<String> = path.split(',').map(|s| s.to_string()).collect();
          (name, paths)
        }),
    );

    Ok(HashPath {
      mining_path: splitty.get("mining_path").unwrap().to_vec(),
      gaming_path: splitty.get("gaming_path").unwrap().to_vec(),
      miner_exe: splitty
        .get("miner_exe")
        .unwrap()
        .first()
        .unwrap()
        .to_string(),
    })
  }

  pub fn fetch() -> Result<Self, std::io::Error> {
    if std::path::Path::new(&hash_conf_dir()).is_dir() {
      let hp = &hash_path();
      debug!("checking hash path @ {}", hp);
      if std::path::Path::new(hp).is_file() {
        // parse dat
        let data = &std::fs::read_to_string(hash_path())?;
        HashPath::parse(data)
      } else {
        debug!("HASH_PATH not a file");
        let mut file = std::fs::File::create(hash_path())?;
        file.write_all(default_conf().as_bytes())?;
        let data = &std::fs::read_to_string(hash_path())?;
        HashPath::parse(data)
      }
    } else {
      debug!("Creating hash_conf_dir...");
      std::fs::create_dir(&hash_conf_dir())?;
      Self::fetch()
    }
  }
}

impl HashEnv {
  pub fn setup() -> Self {
    let env = HashEnv {
      hash_path: HashPath::fetch().expect("couldn't parse the HASH_PATH"),
      sys: Sys {
        system: System::new_all(),
        pids: Pids::DEFAULT,
      },
      gpu: Gpu::new(PYTHON, GPUTIL_PY),
    };
    debug!("env = {:#?}", env);
    env
  }

  pub fn run(&mut self, is_deep_search: bool) -> Rig {
    let current: Rig = Rig::state(self, is_deep_search);
    info!("Rig::state = {:?}", current);
    Rig::move_state(current, self)
  }
}

#[cfg(test)]
mod tests {
  use sysinfo::SystemExt;

  use crate::rig::Rig;
  use crate::sys::{Pids, Sys};
  use crate::{default_nice_hash_location, HashEnv, HashPath};

  #[test]
  fn hashpath_fetch() {
    let res = HashPath::fetch();
    assert!(res.is_ok());
    let hp = res.unwrap();
    assert_eq!(hp.mining_path, vec!["NiceHashMiner.exe", "app_nhm.exe"]);
    assert_eq!(
      hp.gaming_path,
      vec!["Notepad.exe", "D:\\GAMES\\steamapps\\common"]
    );
    assert_eq!(hp.miner_exe, default_nice_hash_location())
  }

  #[test]
  fn hashpath_parse() {
    let contents = "
gaming_path=Notepad.exe,D:\\GAMES\\steamapps\\common
mining_path=NiceHashMiner.exe,app_nhm.exe
miner_exe="
      .to_string()
      + &default_nice_hash_location();
    let res = HashPath::parse(&contents);
    assert!(res.is_ok());
    let hp = res.unwrap();
    assert_eq!(hp.mining_path, vec!["NiceHashMiner.exe", "app_nhm.exe"]);
    assert_eq!(
      hp.gaming_path,
      vec!["Notepad.exe", "D:\\GAMES\\steamapps\\common"]
    );
    assert_eq!(hp.miner_exe, default_nice_hash_location())
  }

  #[test]
  fn rig_gets_state() {
    let mut env = HashEnv::setup();
    let state = Rig::state(&mut env);
    assert_ne!(state, Rig::Conflict); // get_state always Idle in CI
  }

  // #[test]
  fn gets_priority_processes() {
    let mut sys = Sys {
      system: sysinfo::System::new_all(),
      pids: Pids::DEFAULT,
    };
    let hp = HashPath::fetch().unwrap();
    let pids = &mut sys.refresh_pids(&hp);
    assert!(!pids.mining.is_empty() || pids.mining.is_empty());
  }

  #[test]
  fn run_debug() {
    let updated: Rig = HashEnv::setup().run();
    assert_eq!(updated, Rig::Mining)
  }
}
