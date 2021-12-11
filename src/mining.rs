use crate::gpu::{GPU, WindowsGPU};
use crate::rig::{Rig, RigProcess};

pub struct Mining;

impl Mining {
    pub fn is_healthy(gpu: &WindowsGPU) -> bool {
        return match gpu.get_util() {
            Ok(util) => { util.load > 0.5 && Self::is_process_running() }
            Err(_) => { false }
        };
    }

    pub fn is_process_running() -> bool {
        Rig::filter_processes("nicehash") > 0
    }
}
