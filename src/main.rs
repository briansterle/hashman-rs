use crate::gpu::{GPU, WindowsGPU};
use crate::rig::{Rig, RigProcess};
mod gpu;
mod rig;
mod mining;

use sysinfo::{ProcessExt, System, SystemExt};
use crate::mining::Mining;


fn main() {
    let wgpu: WindowsGPU = GPU::new("src/python/get_gpu_util.py", "python");

    let rig_state = Rig::current_state(&wgpu);
    println!("Rig::current_state {:?}", rig_state);

}
