use crate::state_machine_2::event::Event;
use crate::state_machine_2::state_machine::state_operator::StateOperator;
use crate::state_machine_2::state_machine::state_trait::StateTrait;
use crate::state_machine_2::state_machine::typestate::Typestate;
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
    write!(f, "EJECTED")
  }
}
