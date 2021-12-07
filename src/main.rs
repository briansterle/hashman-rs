use crate::gpu::{GPU, WindowsGPU};

mod gpu;

#[derive(Debug)]
struct GPULoad(f64);


fn util() -> GPULoad {
    GPULoad(42.0)
}

fn main() {
    let wgpu: WindowsGPU = GPU::new("get_gpu_util.py");

    println!("{:?}", wgpu.util());

    wgpu.run_python();
    // wgpu.run

}
