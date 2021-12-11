use std::process::Command;
use crate::Config;
use crate::gpu::{GPU, WindowsGPU};
use crate::rig::{Rig, RigProcess, RigState};

pub struct Mining;


impl Mining {

    pub fn restart(config: &Config) -> Result<RigState, RigState> {
        let output = Command::new(&config.miner_exe)
            .output()
            .expect("failed to start mining process");

        match output.status.code() {
            Some(code) if code == 0 => {
                Ok(RigState::Mining(None))
            }
            Some(code) => {
                println!("Unexpectedly exited mining exe with status code: {}", code);
                Err(RigState::Idle(None))
            }
            None => Err(RigState::Idle(None))
        }
    }

    pub fn is_healthy(gpu: &WindowsGPU) -> bool {
        match gpu.get_util() {
            Ok(util) => { util.load > 0.5 && Self::is_process_running() }
            Err(_) => { false }
        }
    }

    pub fn is_process_running() -> bool {
        Rig::filter_processes("nicehash") > 0
    }
}
