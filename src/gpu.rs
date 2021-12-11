use std::process::Command;

#[derive(Debug)]
pub struct GPULoad {
    pub load: f32
}


pub trait GPU {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(py_file: &'static str, py_exec: &'static str,) -> Self;

    fn get_util(&self) -> Result<GPULoad, String>;
    fn parse_usage(stdout: Vec<u8>) -> f32 {
        return String::from_utf8_lossy(&stdout)
            .trim()
            .parse()
            .unwrap();
    }
}

pub struct WindowsGPU {
    py_file: &'static str,
    py_exec: &'static str,
}

impl GPU for WindowsGPU {
    fn new(py_file: &'static str, py_exec: &'static str,) -> WindowsGPU {
        WindowsGPU { py_file, py_exec }
    }

    fn get_util(&self) -> Result<GPULoad, String> {
        let output = Command::new(self.py_exec)
            .args([self.py_file])
            .output()
            .expect("failed to execute process");

        match output.status.code() {
            Some(code) if code == 0 => Ok(
                GPULoad {
                    load: WindowsGPU::parse_usage(output.stdout)
                }
            ),
            Some(_) => Err(String::from("Exited with non-zero code")),
            None => Err(String::from("Exited with missing code"))
        }
    }
}
