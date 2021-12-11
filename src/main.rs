use crate::config::Config;
use crate::gpu::{GPU, WindowsGPU};
use crate::rig::Rig;

mod gpu;
mod rig;
mod mining;
mod config;

fn main() {
    let wgpu: WindowsGPU = GPU::new("src/python/get_gpu_util.py", "python");

    let rig_state = Rig::current_state(&wgpu);
    println!("Rig::current_state {:?}", rig_state);

    println!("{}", config::read());
    let conf: Config = config::json();
    println!("{:?}", conf);
    let update = Rig::update_state(rig_state, &conf);
    println!("Rig::updated_state {:?}", update);
}
