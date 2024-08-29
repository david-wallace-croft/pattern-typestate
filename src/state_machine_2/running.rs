use super::request::Request;
use super::state_operator::StateOperator;
use super::state_trait::StateTrait;
use super::stopped::StoppedState;
use super::typestate::Typestate;
use crate::state_machine_2::data::Data;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct RunningState;

impl StateTrait for RunningState {}

impl StateOperator<RunningState> {
  pub fn new() -> Self {
    StateOperator {
      state: PhantomData,
    }
  }

  fn skip(
    &self,
    data: &mut Data,
    delta: isize,
  ) -> StateOperator<RunningState> {
    data.position = data
      .position
      .saturating_add_signed(delta);

    StateOperator::<RunningState>::new()
  }

  fn stop(&self) -> StateOperator<StoppedState> {
    StateOperator::<StoppedState>::default()
  }

  pub fn transit(
    self,
    data: &mut Data,
    request: Request,
  ) -> Typestate {
    match request {
      Request::Eject | Request::Reset | Request::Run => {
        Typestate::Running(StateOperator::<RunningState>::new())
      },
      Request::Skip(delta) => Typestate::Running(self.skip(data, delta)),
      Request::Stop => Typestate::Stopped(self.stop()),
    }
  }
}

impl Display for StateOperator<RunningState> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "RUNNING")
  }
}
