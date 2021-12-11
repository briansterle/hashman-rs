use crate::config::Config;
use crate::gpu::{GPU, WindowsGPU};
use crate::rig::{Rig, RigState};

mod gpu;
mod rig;
mod mining;
mod config;

fn main() {
    println!("Hashman [INFO] Reading config...");    
    let conf: Config = config::json();

    println!("Hashman [INFO] config: {:?}", conf);
    let wgpu: WindowsGPU = GPU::new(conf.py_gputil.clone(), conf.py_exec.clone());

    let current: RigState = Rig::current_state(&wgpu);
    println!("Hashman [INFO] Rig::current_state {:?}", current);

    let updated: RigState = Rig::update_state(current, &conf);
    println!("Hashman [INFO] Rig::updated_state {:?}", updated);
}
