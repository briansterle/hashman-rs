use std::collections::hash_map::Values;
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
    fn filter_processes(str: &str) -> usize {
        sysinfo::System::new_all().processes().values()
            .map(|p| String::from(p.name()))
            .filter(|s| s.contains(str))
            .count()
    }

    fn is_mining() -> bool {
        Self::filter_processes("nicehash") > 0
    }
}