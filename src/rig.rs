use core::time;
use std::process::Command;
use std::thread;

use sysinfo::Pid;

use crate::gpu::GPU;
use crate::mining::Mining;
use crate::{HashEnv, Sys, WindowsGPU};

#[derive(Debug, PartialEq)]
pub enum Rig {
  Idle,
  Mining,
  Gaming,
  Conflict { gaming: Vec<Pid>, mining: Vec<Pid> },
}

impl Rig {
  fn on_idle<F>(self, gpu: &WindowsGPU, idle_handler: F, wait_for_boot: bool) -> Self
  where
    F: FnOnce() -> (),
  {
    let max_tries = 90;
    let mut tries = 0;
    let mut return_state = Self::Idle;

    if wait_for_boot {
      while tries < max_tries {
        if gpu.is_hot() {
          return_state = self;
          break;
        } else {
          println!("Sleeping until gpu_load.is_hot...");
          thread::sleep(time::Duration::from_millis(1000));
        }
        tries += 1;
      }
    }

    if return_state == Self::Idle {
      idle_handler();
    }
    return_state
  }

  pub fn state(env: &mut HashEnv) -> Self {
    let (gaming_ps, mining_ps) = env.sys.priority_processes();

    match (gaming_ps.is_empty(), mining_ps.is_empty()) {
      (true, true) => Self::Idle,
      (true, false) => Self::Mining.on_idle(
        &env.gpu,
        || Mining::kill_processes(&mut env.sys, vec![]),
        true,
      ),
      (false, false) => Self::Conflict {
        gaming: Sys::pids(gaming_ps),
        mining: Sys::pids(mining_ps),
      },
      (false, true) => Self::Gaming.on_idle(
        &env.gpu,
        || {
          println!(
            "{:?}",
            gaming_ps.into_iter().map(|p| Sys::pretty_proc(p, "gaming"))
          )
        },
        false,
      ),
    }
  }

  pub fn move_state(current: Rig, env: &mut HashEnv) -> Self {
    match current {
      Self::Idle => Mining::restart_mining(&env.conf.miner_exe),
      // .expect("failed to restart miner.exe"),
      Self::Mining => current,
      Self::Gaming => current,
      Self::Conflict { gaming: _, mining } => {
        Mining::kill_processes(&mut env.sys, mining);
        let mut mining_pids = env.sys.mining_pids();
        while !mining_pids.is_empty() {
          println!("mining_pids still live: {:?}", mining_pids);
          Mining::kill_processes(&mut env.sys, mining_pids.to_owned());
          mining_pids = env.sys.mining_pids();
        }

        Self::Gaming
      }
    }
  }
}
