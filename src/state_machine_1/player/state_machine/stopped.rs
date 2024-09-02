use super::ejected::EjectedState;
use super::running::RunningState;
use super::state_trait::StateTrait;
use std::fmt::{Display, Formatter, Result};

const STATE_NAME: &str = "STOPPED";

#[derive(Debug, PartialEq)]
pub struct StoppedState {
  position: usize,
}

impl StoppedState {
  pub fn eject(self) -> EjectedState {
    EjectedState::new(self.position)
  }

  pub fn new(position: usize) -> Self {
    Self {
      position,
    }
  }

  pub fn reset(&mut self) {
    self.position = 0;
  }

  pub fn run(self) -> RunningState {
    RunningState::new(self.position)
  }
}

impl Display for StoppedState {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "{STATE_NAME}")
  }
}

impl StateTrait for StoppedState {
  fn get_position(&self) -> usize {
    self.position
  }

  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
