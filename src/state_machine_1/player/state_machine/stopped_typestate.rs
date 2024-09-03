use super::super::data::Data;
use super::ejected_typestate::EjectedTypestate;
use super::running_typestate::RunningTypestate;
use super::typestate_trait::TypestateTrait;

const STATE_NAME: &str = "STOPPED";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StoppedTypestate;

impl StoppedTypestate {
  pub fn eject(self) -> EjectedTypestate {
    EjectedTypestate
  }

  pub fn reset(
    &self,
    data: &mut Data,
  ) {
    data.position = 0;
  }

  pub fn run(self) -> RunningTypestate {
    RunningTypestate
  }
}

impl TypestateTrait for StoppedTypestate {
  fn get_state_name() -> &'static str {
    STATE_NAME
  }
}
