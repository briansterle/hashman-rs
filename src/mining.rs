
use std::process::Command;

use sysinfo::{ProcessExt, Signal, SystemExt};

use crate::Config;
use crate::gpu::{GPU, GPULoad, WindowsGPU};
use crate::rig::{RigState};

pub struct Mining;


fn background_process(process: &String) -> String {
    let mut prefix = "START /B ".to_owned();
    prefix.push_str(process);
    prefix
}


impl Mining {
    pub fn restart(config: &Config) -> Result<RigState, RigState> {
        let process = background_process(&config.miner_exe);

        let output = Command::new(process)
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
        let load: GPULoad = gpu.get_util().expect("error getting gpu util");

        match sysinfo::System::new_all()
            .processes()
            .values()
            .find(|p| p.name().to_lowercase().contains("nicehash")
            ) {
            Some(_p) if (load.is_hot()) => {
                RigState::Mining(false)
            }
            Some(p) => { // not hot, but mining
                RigState::Mining(p.kill(Signal::Kill))
            }
            None if load.is_hot() => {
                RigState::Gaming(false)
            }
            None => { // not hot, not mining
                RigState::Idle(false)
            }
        }
    }
}
