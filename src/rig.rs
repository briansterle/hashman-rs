use crate::{Config, WindowsGPU};
use crate::mining::Mining;

#[derive(Debug)]
pub enum Rig {
    Idle(bool),
    Mining(bool),
    Gaming(bool),
}

impl Rig {
    pub fn move_state(self, config: &Config) -> Rig {
        if self == Rig::Idle(false) {
            Mining::restart_async(config).expect("oops")
        } else {
            self
        }
    }
}

impl PartialEq for Rig {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

