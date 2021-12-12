use std::process::Command;

use crossbeam::thread;
use sysinfo::{ProcessExt, Signal, SystemExt};

use crate::Config;
use crate::gpu::{GPU, GPULoad, WindowsGPU};
use crate::rig::RigState;

pub struct Mining;


const NICE_HASH_BINARY: &'static str = "app_nhm";

impl Mining {
    pub fn restart_async(config: &Config) -> Result<RigState, ()> {
        thread::scope(|scope| {
            scope.spawn(move |_| {
                Mining::restart(config)
            });
        }).unwrap();
        Ok(RigState::Mining(false))
    }

    pub fn restart(config: &Config) -> Result<RigState, RigState> {
        let output = Command::new(&config.miner_exe)
            .output()
            .expect("failed to start mining process");
        match output.status.code() {
            Some(code) if code == 0 => {
                Ok(RigState::Mining(false))
            }
            Some(code) => {
                println!("Unexpectedly exited mining exe with status code: {}", code);
                Err(RigState::Idle(false))
            }
            None => Err(RigState::Idle(false))
        }
    }

    pub fn kill_all() {
        let system = sysinfo::System::new_all();
        let pid_map = system.processes();
        let ps = pid_map.values();
        ps
            .filter(|p| p.name().to_lowercase().contains(NICE_HASH_BINARY))
            .for_each(|p| {
                let kill = p.kill(Signal::Kill);
                let s = sysinfo::System::new();
                // ensure proc is killed
                assert!(kill && s.process(p.pid()).is_none());
            })
    }

    pub fn run(gpu: &WindowsGPU) -> RigState {
        let load: GPULoad = gpu.get_util().expect("error getting gpu util");

        // DEBUG
        // sysinfo::System::new_all().processes().values().for_each(|x| println!("{:?}", x));

        match sysinfo::System::new_all()
            .processes()
            .values()
            .find(|p| p.name().to_lowercase().contains(NICE_HASH_BINARY)
            ) {
            Some(_) if (load.is_hot()) => {
                println!("hot & mining");
                RigState::Mining(false)
            }
            Some(_) => { // not hot, but mining
                println!("not hot, but Failure[Mining]");
                Mining::kill_all();
                RigState::Mining(true)
            }
            None if load.is_hot() => {
                println!("hot & gaming");
                RigState::Gaming(false)
            }
            None => { // not hot, not mining
                println!("system idle");
                RigState::Idle(false)
            }
        }
    }
}

