use std::{thread as std_thread, time};
use std::collections::HashSet;
use std::process::Command;

use crossbeam::thread;
use sysinfo::{ProcessExt, Signal, SystemExt};

use crate::Config;
use crate::gpu::{GPU, GPULoad, WindowsGPU};
use crate::rig::Rig;

pub struct Mining;


const NICE_HASH_BINARY: &'static str = "app_nhm";

fn get_hash_bins() -> HashSet<&'static str> { HashSet::from(["app_nhm", "nicehash", "hash", "nice"]) }

impl Mining {
    pub fn restart_async(config: &Config) -> Result<Rig, ()> {
        thread::scope(|scope| {
            scope.spawn(move |_| {
                Mining::restart(config)
            });
        }).unwrap();
        Ok(Rig::Mining(false))
    }

    pub fn restart(config: &Config) -> Result<Rig, Rig> {
        let output = Command::new(&config.miner_exe)
            .output()
            .expect("failed to start mining process");
        match output.status.code() {
            Some(code) if code == 0 => {
                Ok(Rig::Mining(false))
            }
            Some(code) => {
                println!("Unexpectedly exited mining exe with status code: {}", code);
                Err(Rig::Idle(false))
            }
            None => Err(Rig::Idle(false))
        }
    }

    pub fn is_hash_binary(proc_name: &str) -> bool {
        get_hash_bins()
            .iter()
            .any(|bin| proc_name.to_lowercase().contains(proc_name))
    }

    pub fn kill_all() -> bool {
        let system = sysinfo::System::new_all();
        let pid_map = system.processes();
        let ps = pid_map.values();
        let mut kill = false;
        ps
            .filter(|p| Mining::is_hash_binary(p.name()))
            .for_each(|p| {
                let s = sysinfo::System::new();
                while !s.process(p.pid()).is_none() {
                    kill |= p.kill(Signal::Kill);
                    std_thread::sleep(time::Duration::from_millis(420));
                }

                // ensure proc is killed
            });
        kill
    }

    pub fn get_state(gpu: &WindowsGPU) -> Rig {
        let load: GPULoad = gpu.get_util().expect("error getting gpu util");

        // DEBUG
        // sysinfo::System::new_all().processes().values().for_each(|x| println!("{:?}", x));

        let sys = sysinfo::System::new_all();
        match sys
            .processes()
            .values()
            .find(|p| p.name().to_lowercase().contains(NICE_HASH_BINARY)
            ) {
            Some(_) if (load.is_hot()) => {
                println!("hot & mining");
                Rig::Mining(false)
            }
            Some(_) => { // not hot, but mining
                println!("not hot, but Failure[Mining]");
                Mining::kill_all();
                Rig::Mining(true)
            }
            None if load.is_hot() => {
                println!("hot & gaming");
                Rig::Gaming(false)
            }
            None => { // not hot, not mining
                println!("system idle");
                Rig::Idle(false)
            }
        }
    }
}

