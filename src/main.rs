use crate::config::Config;
use crate::gpu::{GPU, WindowsGPU};
use crate::mining::Mining;
use crate::rig::Rig;

mod gpu;
mod rig;
mod mining;
mod config;
mod test;

fn main() {
    println!("Hashman [INFO] Reading config...");
    let conf: Config = config::json();

    println!("Hashman [INFO] config: {:?}", conf);
    let wgpu: WindowsGPU = GPU::new(conf.py_gputil.clone(), conf.py_exec.clone());

    let current: Rig = Rig::get_state(&wgpu);
    println!("Hashman [INFO] Rig::get_state {:?}", current);

    let updated: Rig = current.move_state(&conf);
    println!("Hashman [INFO] rig::move_state {:?}", updated);
}
