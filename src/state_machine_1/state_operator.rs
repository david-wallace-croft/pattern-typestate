use super::state_trait::StateTrait;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct StateOperator<S: StateTrait> {
  // TODO: Make this private
  pub state: S,
}

impl<S: StateTrait> StateOperator<S> {
  pub fn get_position(&self) -> usize {
    self
      .state
      .get_position()
  }
}

impl<S: StateTrait> Display for StateOperator<S> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    self
      .state
      .fmt(f)
  }
}
