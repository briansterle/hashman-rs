use std::borrow::Borrow;
use std::process::Command;
use sysinfo::{Process, ProcessExt, Signal, SystemExt};
use crate::Config;
use crate::gpu::{GPU, GPULoad, WindowsGPU};
use crate::rig::{Rig, RigProcess, RigState};

use sysinfo::{System};

pub struct Mining;


impl Mining {
    pub fn restart(config: &Config) -> Result<RigState, RigState> {
        let output = Command::new(&config.miner_exe)
            .output()
            .expect("failed to start mining process");

        match output.status.code() {
            Some(code) if code == 0 => {
                Ok(RigState::Mining(false))
            }
            Some(code) => {
                println!("Unexpectedly exited mining exe with status code: {}", code);
                Err(RigState::Idle(false))
            }
            None => Err(RigState::Idle(false))
        }
    }

    pub fn run(gpu: &WindowsGPU) -> RigState {
        match (sysinfo::System::new_all().processes().values().find(|p|
                    p.name().to_lowercase().contains("nicehash")),
               gpu.get_util()
        ) {
            (Some(p), Ok(u)) if (u.load > 0.5) => {
                RigState::Mining(false)
            }
            (Some(p), Ok(u)) if (u.load <= 0.5) => {
                RigState::Mining(p.kill(Signal::Kill))
            }
            (Some(p), Err(e)) => {
                RigState::Idle(p.kill(Signal::Kill))
            }
            (None, Ok(u)) => if u.load > 0.5 {
                RigState::Gaming(false)
            } else {
                RigState::Idle(false)
            }

            (None, Err(e)) => {
                RigState::Idle(false)
            }

            _ => unreachable!()
        }
    }
}
