use super::super::state_machine::ejected::EjectedState;
use super::super::state_machine::running::RunningState;
use super::super::state_machine::state_operator::StateOperator;
use super::super::state_machine::state_trait::StateTrait;
use super::super::state_machine::stopped::StoppedState;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(StateOperator<EjectedState>),
  Running(StateOperator<RunningState>),
  Stopped(StateOperator<StoppedState>),
}

impl Typestate {
  pub fn get_state_name(&self) -> &'static str {
    match self {
      Typestate::Ejected(_state_operator) => EjectedState::get_state_name(),
      Typestate::Running(_state_operator) => RunningState::get_state_name(),
      Typestate::Stopped(_state_operator) => StoppedState::get_state_name(),
    }
  }
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
