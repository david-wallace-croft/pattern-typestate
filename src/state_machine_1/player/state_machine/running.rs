use super::state_trait::StateTrait;
use super::stopped::StoppedState;
use std::fmt::{Display, Formatter};

const STATE_NAME: &str = "RUNNING";

#[derive(Debug, PartialEq)]
pub struct RunningState {
  position: usize,
}

impl RunningState {
  pub fn new(position: usize) -> Self {
    Self {
      position,
    }
  }

  pub fn skip(
    &mut self,
    delta: isize,
  ) {
    self.position = self
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> StoppedState {
    StoppedState::new(self.position)
  }
}

impl Display for RunningState {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{STATE_NAME}")
  }
}

impl StateTrait for RunningState {
  fn get_position(&self) -> usize {
    self.position
  }

  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
