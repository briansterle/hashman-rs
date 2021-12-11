use sysinfo;
use sysinfo::{Process, ProcessExt, SystemExt};

use crate::{Config, GPU, WindowsGPU};
use crate::mining::Mining;
use crate::rig::RigState::Idle;

#[derive(Debug)]
// #[derive(PartialEq)]
pub enum RigState {
    Idle(Option<Process>),
    Mining(Option<Process>),
    Gaming(Option<Process>),
}

impl PartialEq for RigState {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

pub struct Rig;

impl Rig {
    pub fn update_state(current: RigState, config: &Config) -> RigState {
        if current == RigState::Idle(None) {
            Mining::restart(config).expect("oops")
        } else {
            current
        }
    }

    pub fn current_state(gpu: &WindowsGPU) -> RigState {
        if Mining::is_healthy(gpu) {
            RigState::Mining(None)
        } else if gpu.get_util().expect("err running gputil.py").load > 0.5 {
            RigState::Gaming(None)
        } else {
            RigState::Idle(None)
        }
    }
}

impl RigProcess for Rig {}

pub trait RigProcess {
    fn filter_processes(str: &str) -> usize {
        sysinfo::System::new_all().processes().values()
            .filter(|p| p.name().to_lowercase().contains(&str.to_lowercase()))
            .count()
    }
}