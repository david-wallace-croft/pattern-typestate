use super::request::Request;
use super::state_operator::StateOperator;
use super::state_trait::StateTrait;
use super::typestate::Typestate;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct EjectedState {
  position: usize,
}

impl Display for EjectedState {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "EJECTED")
  }
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
