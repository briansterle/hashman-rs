use crate::config::Config;
use crate::gpu::{GPU, WindowsGPU};
use crate::rig::Rig;

mod gpu;
mod rig;
mod mining;
mod config;

fn main() {
    println("Hashman [INFO] Reading config...")    
    let conf: Config = config::json();

    let wgpu: WindowsGPU = GPU::new(&conf.py_exec, &conf.py_gputil);

    let rig_state = Rig::current_state(&wgpu);
    println!("Rig::current_state {:?}", rig_state);

    println!("{:?}", conf);
    let update = Rig::update_state(rig_state, &conf);
    println!("Rig::updated_state {:?}", update);
}
