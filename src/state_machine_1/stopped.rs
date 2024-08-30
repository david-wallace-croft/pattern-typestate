use super::ejected::EjectedState;
use super::request::Request;
use super::running::RunningState;
use super::state_operator::StateOperator;
use super::state_trait::StateTrait;
use super::typestate::Typestate;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct StoppedState {
  // TODO: Make this private
  pub position: usize,
}

impl Display for StoppedState {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "STOPPED")
  }
}

impl StateTrait for StoppedState {
  fn get_position(&self) -> usize {
    self.position
  }
}

impl StateOperator<StoppedState> {
  pub fn eject(self) -> StateOperator<EjectedState> {
    StateOperator::<EjectedState>::new(
      self
        .state
        .position,
    )
  }

  pub fn new(position: usize) -> Self {
    StateOperator {
      state: StoppedState {
        position,
      },
    }
  }

  pub fn reset(&mut self) {
    self
      .state
      .position = 0;
  }

  pub fn run(self) -> StateOperator<RunningState> {
    StateOperator::<RunningState>::new(
      self
        .state
        .position,
    )
  }

  pub fn transit(
    mut self,
    request: &Request,
  ) -> Typestate {
    match request {
      Request::Eject => Typestate::Ejected(self.eject()),
      Request::Reset => {
        self.reset();

        Typestate::Stopped(self)
      },
      Request::Run => Typestate::Running(self.run()),
      Request::Skip(_) | Request::Stop => Typestate::Stopped(self),
    }
  }
}
