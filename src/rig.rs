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
            Mining::restart()
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
        println!("Filtering processes");
        sysinfo::System::new_all().processes().values()
            .map(|p| {
                println!("{:?}", p);
                p
            })
            .map(|p| String::from(p.name()))
            .filter(|s| s.contains(str))
            .count()
    }
}