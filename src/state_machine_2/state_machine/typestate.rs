use crate::state_machine_2::state_machine::ejected::EjectedState;
use crate::state_machine_2::state_machine::running::RunningState;
use crate::state_machine_2::state_machine::state_operator::StateOperator;
use crate::state_machine_2::state_machine::stopped::StoppedState;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(StateOperator<EjectedState>),
  Running(StateOperator<RunningState>),
  Stopped(StateOperator<StoppedState>),
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::Stopped(StateOperator::<StoppedState>::default())
  }
}

impl Display for Typestate {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Typestate::Ejected(state_operator) => state_operator.fmt(f),
      Typestate::Running(state_operator) => state_operator.fmt(f),
      Typestate::Stopped(state_operator) => state_operator.fmt(f),
    }
  }
}
