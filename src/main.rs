use crate::gpu::{GPU, WindowsGPU};
use crate::rig::{Rig};
mod gpu;
mod rig;


fn main() {
    let wgpu: WindowsGPU = GPU::new("src/python/get_gpu_util.py", "python");
    println!("{:?}", wgpu.get_util());

    let rig_state = Rig::current_state();
    println!("{:?}", rig_state);
}
