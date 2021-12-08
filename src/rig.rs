
#[derive(Debug)]
pub enum RigState {
    Idle,
    Mining,
    Gaming,
}

pub struct Rig;

impl Rig {

    pub fn current_state() -> RigState {
        RigState::Idle
    }

}