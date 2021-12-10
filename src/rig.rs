use std::collections::HashMap;
use crate::gpu::GPULoad;
use crate::mining::Mining;

use std::process;
use std::process::id;

use sysinfo;
use sysinfo::{Pid, Process, ProcessExt, SystemExt};

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

impl RigProcess for Rig {

}

pub trait RigProcess {
    fn processes_matching(str: &str) -> &HashMap<Pid, Process> {
        return sysinfo::System::new_all().processes().into_iter()
            .filter(|&(_, p)| p.name().contains(str))
            .collect();
    }
}