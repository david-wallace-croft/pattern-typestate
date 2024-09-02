use super::super::data::Data;
use super::ejected_state::EjectedState;
use super::running_state::RunningState;
use super::state_trait::StateTrait;

const STATE_NAME: &str = "STOPPED";

#[derive(Debug, PartialEq)]
pub struct StoppedState;

impl StoppedState {
  pub fn eject(self) -> EjectedState {
    EjectedState
  }

  pub fn reset(
    &self,
    data: &mut Data,
  ) {
    data.position = 0;
  }

  pub fn run(self) -> RunningState {
    RunningState
  }
}

impl StateTrait for StoppedState {
  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
