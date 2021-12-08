use crate::gpu::GPULoad;


pub struct Mining;

impl Mining {
    pub fn is_active(gpu_load: GPULoad) -> bool {
        return gpu_load.load > 0.5;
    }
}
