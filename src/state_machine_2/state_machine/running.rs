use crate::state_machine_2::data::Data;
use crate::state_machine_2::event::Event;
use crate::state_machine_2::state_machine::state_operator::StateOperator;
use crate::state_machine_2::state_machine::state_trait::StateTrait;
use crate::state_machine_2::state_machine::stopped::StoppedState;
use crate::state_machine_2::state_machine::typestate::Typestate;
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
    event: Event,
  ) -> Typestate {
    match event {
      Event::Eject | Event::Reset | Event::Run => {
        Typestate::Running(StateOperator::<RunningState>::new())
      },
      Event::Skip(delta) => Typestate::Running(self.skip(data, delta)),
      Event::Stop => Typestate::Stopped(self.stop()),
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