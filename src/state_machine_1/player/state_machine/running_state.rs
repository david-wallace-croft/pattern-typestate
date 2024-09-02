use super::super::data::Data;
use super::state_trait::StateTrait;
use super::stopped_state::StoppedState;

const STATE_NAME: &str = "RUNNING";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunningState;

impl RunningState {
  pub fn skip(
    &self,
    data: &mut Data,
    delta: isize,
  ) {
    data.position = data
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> StoppedState {
    StoppedState
  }
}

impl StateTrait for RunningState {
  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
