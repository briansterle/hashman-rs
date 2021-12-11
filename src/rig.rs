use sysinfo;
use sysinfo::{ProcessExt, SystemExt};

use crate::{GPU, WindowsGPU};
use crate::mining::Mining;
use crate::rig::RigState::Idle;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RigState {
    Idle,
    Mining,
    Gaming,
}

pub struct Rig;

impl Rig {

    pub fn update_state(current: RigState) -> RigState {
        if current == Idle {
            Mining::restart().expect("oops")
        } else {
            current
        }
    }

    pub fn current_state(gpu: &WindowsGPU) -> RigState {
        if Mining::is_healthy(gpu) {
            RigState::Mining
        } else if gpu.get_util().expect("oops").load > 0.5 {
            RigState::Gaming
        } else {
            RigState::Idle
        }
    }

}

impl RigProcess for Rig {

}

pub trait RigProcess {
    fn filter_processes(str: &str) -> usize {
        sysinfo::System::new_all().processes().values()
            .filter(|p| p.name().to_lowercase().contains(&str.to_lowercase()))
            .count()
    }
}