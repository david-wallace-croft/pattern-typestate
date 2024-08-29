use super::request::Request;
use super::state_operator::StateOperator;
use super::state_trait::StateTrait;
use super::typestate::Typestate;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct EjectedState;

impl StateTrait for EjectedState {}

impl StateOperator<EjectedState> {
  pub fn new() -> Self {
    StateOperator {
      state: PhantomData,
    }
  }

  pub fn transit(
    self,
    _request: Request,
  ) -> Typestate {
    Typestate::Ejected(self)
  }
}

impl Display for StateOperator<EjectedState> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "EJECTED")
  }
}
