use super::ejected::EjectedState;
use super::request::Request;
use super::running::RunningState;
use super::stopped::StoppedState;
use crate::example4::state_operator::StateOperator;
use crate::example4::state_trait::StateTrait;

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(StateOperator<EjectedState>),
  Running(StateOperator<RunningState>),
  Stopped(StateOperator<StoppedState>),
}

impl Typestate {
  pub fn transit(
    self,
    request: &Request,
  ) -> Self {
    match self {
      Typestate::Ejected(state_operator) => state_operator.transit(request),
      Typestate::Running(state_operator) => state_operator.transit(request),
      Typestate::Stopped(state_operator) => state_operator.transit(request),
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::Stopped(StateOperator::<StoppedState>::new(0))
  }
}

impl StateTrait for Typestate {
  fn get_position(&self) -> usize {
    match self {
      Typestate::Ejected(state_operator) => state_operator.get_position(),
      Typestate::Running(state_operator) => state_operator.get_position(),
      Typestate::Stopped(state_operator) => state_operator.get_position(),
    }
  }
}
