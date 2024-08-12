use super::request::Request;
use super::state_operator::StateOperator;
use super::state_trait::StateTrait;
use super::typestate::Typestate;

#[derive(Debug, PartialEq)]
pub struct EjectedState {
  position: usize,
}

impl StateTrait for EjectedState {
  fn get_position(&self) -> usize {
    self.position
  }
}

impl StateOperator<EjectedState> {
  pub fn new(position: usize) -> Self {
    StateOperator {
      state: EjectedState {
        position,
      },
    }
  }

  pub fn transit(
    self,
    _request: &Request,
  ) -> Typestate {
    Typestate::Ejected(self)
  }
}
