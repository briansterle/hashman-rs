use crate::gpu::{GPU, WindowsGPU};
use crate::rig::{Rig};
mod gpu;
mod rig;
mod mining;


fn main() {
    let wgpu: WindowsGPU = GPU::new("src/python/get_gpu_util.py", "python");
    let util = wgpu.get_util();
    println!("{:?}", util);

    let rig_state = Rig::current_state(util.expect("no gpu util found"));
    println!("{:?}", rig_state);

    println!("gm")
}
