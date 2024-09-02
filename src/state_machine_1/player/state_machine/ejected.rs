use super::super::event::Event;
use super::super::state_machine::state_operator::StateOperator;
use super::super::state_machine::state_trait::StateTrait;
use super::super::state_machine::typestate::Typestate;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

const STATE_NAME: &str = "EJECTED";

#[derive(Debug, PartialEq)]
pub struct EjectedState;

impl StateTrait for EjectedState {
  fn get_state_name() -> &'static str {
    STATE_NAME
  }
}

impl StateOperator<EjectedState> {
  pub fn new() -> Self {
    StateOperator {
      state: PhantomData,
    }
  }

  pub fn transit(
    self,
    _event: Event,
  ) -> Typestate {
    Typestate::Ejected(self)
  }
}

impl Display for StateOperator<EjectedState> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{STATE_NAME}")
  }
}
