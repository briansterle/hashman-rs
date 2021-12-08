use crate::gpu::{GPU, WindowsGPU};
mod gpu;


fn main() {
    let wgpu: WindowsGPU = GPU::new("src/python/get_gpu_util.py");
    println!("{:?}", wgpu.get_util());
}
