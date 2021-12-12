use sysinfo::{ProcessExt, SystemExt};

use crate::Config;
use crate::gpu::{GPU, GPULoad, WindowsGPU};
use crate::mining::Mining;

#[derive(Debug)]
pub enum Rig {
    Idle(bool),
    Mining(bool),
    Gaming(bool),
}

impl Rig {
    pub fn get_state(gpu: &WindowsGPU) -> Rig {
        let load: GPULoad = gpu.get_util().expect("error getting gpu util");
        let sys = sysinfo::System::new_all();
        match sys
            .processes()
            .values()
            .find(|p| Mining::is_hash_binary(p.name())) {
            Some(_) if (load.is_hot()) => {
                Rig::Mining(false)
            }
            None if load.is_hot() => {
                println!("hot & gaming");
                Rig::Gaming(false)
            }
            Some(_) => { // not hot, but mining
                Mining::kill_all();
                Rig::Idle(true)
            }
            None => { // not hot, not mining
                println!("system idle");
                Rig::Idle(false)
            }
        }
    }
    pub fn move_state(self, config: &Config) -> Rig {
        if self == Rig::Idle(false) {
            Mining::restart_async(config).expect("oops")
        } else {
            self
        }
    }
}

impl PartialEq for Rig {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

