use super::state_trait::StateTrait;
use std::fmt::{Display, Formatter};

const STATE_NAME: &str = "EJECTED";

#[derive(Debug, PartialEq)]
pub struct EjectedState {
  position: usize,
}

impl EjectedState {
  pub fn new(position: usize) -> Self {
    Self {
      position,
    }
  }
}

impl Display for EjectedState {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{STATE_NAME}")
  }
}

impl StateTrait for EjectedState {
  fn get_position(&self) -> usize {
    self.position
  }

  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
