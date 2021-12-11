use std::borrow::Borrow;
use sysinfo;
use sysinfo::{Process, ProcessExt, System, SystemExt};

use crate::{Config, GPU, WindowsGPU};
use crate::mining::Mining;
use crate::rig::RigState::Idle;

#[derive(Debug)]
pub enum RigState {
    Idle(bool),
    Mining(bool),
    Gaming(bool),
}

impl PartialEq for RigState {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

pub struct Rig;

impl Rig {
    pub fn update_state(current: RigState, config: &Config) -> RigState {
        if current == RigState::Idle(false) {
            Mining::restart(config).expect("oops")
        } else {
            current
        }
    }

    pub fn current_state(gpu: &WindowsGPU) -> RigState {
        Mining::run(gpu)
    }
}
