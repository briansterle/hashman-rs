use std::process::Command;

#[derive(Debug)]
#[derive(PartialOrd, PartialEq)]
pub struct GPULoad {
    pub load: f32,
}

impl GPULoad {
    pub fn is_hot(&self) -> bool {
        println!("is_hot? {}", self.load);
        self.load > 0.5
    }
    pub fn not_hot(&self) -> bool {
        !self.is_hot()
    }
}


pub trait GPU {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(py_gputil: String, py_exec: String) -> Self;

    fn get_util(&self) -> Result<GPULoad, String>;
    fn parse_usage(stdout: Vec<u8>) -> f32 {
        return String::from_utf8_lossy(&stdout)
            .trim()
            .parse()
            .unwrap();
    }
}

pub struct WindowsGPU {
    py_gputil: String,
    py_exec: String,
}

impl GPU for WindowsGPU {
    fn new(py_gputil: String, py_exec: String) -> WindowsGPU {
        WindowsGPU { py_gputil, py_exec }
    }

    fn get_util(&self) -> Result<GPULoad, String> {
        let output = Command::new(&self.py_exec)
            .args([&self.py_gputil])
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
