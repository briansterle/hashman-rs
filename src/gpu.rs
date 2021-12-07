use std::process::Command;


#[derive(Debug)]
pub struct GPULoad(f64);


pub trait GPU {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(py_file: &'static str) -> Self;

    fn util(&self) -> GPULoad;
}

pub struct WindowsGPU {
    py_file: &'static str,
    py_exec: &'static str,
}

impl WindowsGPU {
    pub fn run_python(&self) -> Result<(), String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("echo")
                .args(["world", "hello"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("echo")
                .args(["world", "hello"])
                .output()
                .expect("failed to execute process")
        };

        let stdout =  String::from_utf8_lossy(&output.stdout);

        println!("{:?}", stdout);
        match output.status.code() {
            Some(code) if code == 0 => Ok(()),
            Some(_) => Err(String::from("Exited with non-zero code")),
            None => Err(String::from("Exited with missing code"))
        }
    }
}

impl GPU for WindowsGPU {
    fn new(py_file: &'static str) -> WindowsGPU {
        WindowsGPU { py_file, py_exec: "python3" }
    }
    fn util(&self) -> GPULoad {
        GPULoad(42.0)
    }
}
