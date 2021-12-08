use crate::gpu::GPULoad;
use crate::mining::Mining;


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