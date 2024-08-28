use super::state_trait::StateTrait;

#[derive(Debug, PartialEq)]
pub struct StateOperator<S: StateTrait> {
  pub state: S,
}

impl<S: StateTrait> StateOperator<S> {
  pub fn get_position(&self) -> usize {
    self
      .state
      .get_position()
  }
}
