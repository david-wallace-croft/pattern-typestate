use super::ejected::EjectedState;
use super::request::Request;
use super::running::RunningState;
use super::stopped::StoppedState;

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(EjectedState),
  Running(RunningState),
  Stopped(StoppedState),
}

impl Typestate {
  pub fn transit(
    self,
    request: &Request,
  ) -> Self {
    match self {
      Typestate::Ejected(ejected_state) => ejected_state.transit(request),
      Typestate::Running(running_state) => running_state.transit(request),
      Typestate::Stopped(stopped_state) => stopped_state.transit(request),
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::Stopped(StoppedState::new(0))
  }
}
