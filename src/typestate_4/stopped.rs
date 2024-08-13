use super::data::Data;
use super::ejected::EjectedState;
use super::request::Request;
use super::running::RunningState;
use super::state_operator::StateOperator;
use super::state_trait::StateTrait;
use super::typestate::Typestate;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct StoppedState;

impl StateTrait for StoppedState {}

impl StateOperator<StoppedState> {
  fn eject(&self) -> StateOperator<EjectedState> {
    StateOperator::<EjectedState>::new()
  }

  pub fn new() -> Self {
    StateOperator {
      state: PhantomData,
    }
  }

  fn reset(
    &self,
    data: &mut Data,
  ) -> StateOperator<StoppedState> {
    data.position = 0;

    StateOperator::<StoppedState>::new()
  }

  fn run(&self) -> StateOperator<RunningState> {
    StateOperator::<RunningState>::new()
  }

  pub fn transit(
    self,
    data: &mut Data,
    request: &Request,
  ) -> Typestate {
    match request {
      Request::Eject => Typestate::Ejected(self.eject()),
      Request::Reset => Typestate::Stopped(self.reset(data)),
      Request::Run => Typestate::Running(self.run()),
      Request::Skip(_) | Request::Stop => {
        Typestate::Stopped(StateOperator::<StoppedState>::new())
      },
    }
  }
}

impl Display for StateOperator<StoppedState> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "STOPPED")
  }
}
