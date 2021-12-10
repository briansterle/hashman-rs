use crate::gpu::GPULoad;
use crate::mining::Mining;

use std::process;
use std::process::id;

#[derive(Debug)]
pub enum RigState {
    Idle,
    Mining,
    Gaming,
}

pub struct Rig;

impl Rig {

    pub fn current_state(gpu: GPULoad) -> RigState {
        if Mining::is_active(gpu) {
            RigState::Mining
        } else {
            RigState::Idle
        }
    }

}


trait RigProcess {
    fn get_all(&self) -> u32 {
        id()
    }
}