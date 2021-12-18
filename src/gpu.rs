use std::process::Command;

use crate::GPULoad;

pub trait Gpu {
  fn new(py_exec: &str, py_gputil: &str) -> Self;
  fn is_hot(&self) -> bool {
    self.get_util().unwrap() > 0.5
  }
  fn get_util(&self) -> Result<GPULoad, String>;
  fn parse_usage(stdout: Vec<u8>) -> f64 {
    return String::from_utf8_lossy(&stdout).trim().parse().unwrap();
  }
}

#[derive(Debug)]
pub struct WindowsGPU {
  py_exec: String,
  py_gputil: String,
}

impl Gpu for WindowsGPU {
  fn new(py_exec: &str, py_gputil: &str) -> WindowsGPU {
    WindowsGPU {
      py_exec: py_exec.to_string(),
      py_gputil: py_gputil.to_string(),
    }
  }

  fn get_util(&self) -> Result<GPULoad, String> {
    println!("{:?}", self);
    let output = Command::new(&self.py_exec)
      .args(["-c", &self.py_gputil])
      .output()
      .expect("failed to execute process");
    println!("{:?}", output);
    match output.status.code() {
      Some(code) if code == 0 => {
        let load = WindowsGPU::parse_usage(output.stdout);
        println!("gpu_load: {:?}", load);
        Ok(load)
      }
      Some(_) => Err(String::from("Exited with non-zero code")),
      None => Err(String::from("Exited with missing code")),
    }
  }
}
