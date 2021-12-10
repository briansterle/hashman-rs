use crate::gpu::{GPU, WindowsGPU};
use crate::rig::{Rig, RigProcess};
mod gpu;
mod rig;
mod mining;

use sysinfo::{ProcessExt, System, SystemExt};



fn main() {
    let wgpu: WindowsGPU = GPU::new("src/python/get_gpu_util.py", "python");
    let util = wgpu.get_util();
    println!("{:?}", util);

    let rig_state = Rig::current_state(util.expect("no gpu util found"));
    println!("{:?}", rig_state);



    let s = sysinfo::System::new_all();
    for (pid, process) in s.processes() {
        println!("{} {}", pid, process.name());
    }

    let ps = Rig::processes_matching("nicehash");
    ps.iter().for_each(|(pid, p)| println!("{}", pid))

}
