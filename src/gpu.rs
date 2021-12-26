use std::num::ParseFloatError;
use std::process::Command;
use std::{error, fmt};

use log::{debug, error, trace};

use crate::GPULoad;

pub trait Gpu {
  fn new(py_exec: &str, py_gputil: &str) -> Self;
  fn is_hot(&self) -> bool {
    match self.get_util() {
      Ok(util) => util > 0.5,
      Err(err) => {
        error!("Error getting gpu utilization: {:?}", err);
        false
      }
    }
    // self.get_util() > 0.5
  }
  fn get_util(&self) -> Result<GPULoad, Box<dyn error::Error>>;
  fn parse_usage(stdout: Vec<u8>) -> Result<f64, ParseFloatError> {
    String::from_utf8_lossy(&stdout).trim().parse()
  }
}

#[derive(Debug)]
pub struct WindowsGPU {
  py_exec: String,
  py_gputil: String,
}

#[derive(PartialEq, Debug)]
pub enum GpuError {
  NonZeroExitCode(i32),
  MissingExitCode,
}

impl fmt::Display for GpuError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let description = match *self {
      GpuError::NonZeroExitCode(exit) => format!("Exit code is non zero {}", exit),
      GpuError::MissingExitCode => "Exit code not found!".to_string(),
    };
    f.write_str(description.as_str())
  }
}

impl error::Error for GpuError {}

impl Gpu for WindowsGPU {
  fn new(py_exec: &str, py_gputil: &str) -> WindowsGPU {
    WindowsGPU {
      py_exec: py_exec.to_string(),
      py_gputil: py_gputil.to_string(),
    }
  }

  fn get_util(&self) -> Result<GPULoad, Box<dyn error::Error>> {
    trace!("{:?}", self);
    let output = Command::new(&self.py_exec)
      .args(["-c", &self.py_gputil])
      .output()
      .expect("failed to execute process");
    trace!("{:?}", output);
    match output.status.code() {
      Some(code) if code == 0 => {
        let load = WindowsGPU::parse_usage(output.stdout)?;
        debug!("gpu_load: {:#?}", load);
        Ok(load)
      }
      Some(code) => Err(Box::new(GpuError::NonZeroExitCode(code))),
      None => Err(Box::new(GpuError::MissingExitCode)),
    }
  }
}
