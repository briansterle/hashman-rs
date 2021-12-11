use std::collections::hash_map::Values;
use std::collections::HashMap;
use crate::gpu::GPULoad;
use crate::mining::Mining;

use std::process;
use std::process::id;

use sysinfo;
use sysinfo::{Pid, Process, ProcessExt, SystemExt};
use crate::WindowsGPU;

#[derive(Debug)]
pub enum RigState {
    Idle,
    Mining,
    Gaming,
}

pub struct Rig;

impl Rig {



    /*
      def currentState(): Task[RigState] =
    for {
      gpu <- GPU.util()
      state <- Task {
        if Mining.isActive(gpu) then
          RigState.MINING
        else if gpu.util >= 0.5 then
          RigState.GAMING
        else
          RigState.IDLE
      }
    } yield state

  def updateState(): Task[RigState] =
    for {
      prevState: RigState <- currentState()
      newState <-
        if (prevState == RigState.IDLE) {
          Mining.restart(prevState)
        } else {
          Task.succeed(prevState)
        }
    } yield newState
     */

    pub fn current_state(gpu: &WindowsGPU) -> RigState {
        if Mining::is_healthy(gpu) {
            RigState::Mining
        } else {
            RigState::Idle
        }
    }

}

impl RigProcess for Rig {

}

pub trait RigProcess {
    fn filter_processes(str: &str) -> usize {
        sysinfo::System::new_all().processes().values()
            .map(|p| String::from(p.name()))
            .filter(|s| s.contains(str))
            .count()
    }
}