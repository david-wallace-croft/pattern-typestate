use super::super::data::Data;
use super::super::event::Event;
use super::super::state_machine::state_operator::StateOperator;
use super::super::state_machine::state_trait::StateTrait;
use super::super::state_machine::stopped::StoppedState;
use super::super::state_machine::typestate::Typestate;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

const STATE_NAME: &str = "RUNNING";

#[derive(Debug, PartialEq)]
pub struct RunningState;

impl StateTrait for RunningState {
  fn get_state_name() -> &'static str {
    STATE_NAME
  }
}

impl StateOperator<RunningState> {
  fn skip(
    &self,
    data: &mut Data,
    delta: isize,
  ) {
    data.position = data
      .position
      .saturating_add_signed(delta);
  }

  fn stop(self) -> StateOperator<StoppedState> {
    StateOperator::<StoppedState>::default()
  }

  pub fn transit(
    self,
    data: &mut Data,
    event: Event,
  ) -> Typestate {
    match event {
      Event::Eject | Event::Reset | Event::Run => Typestate::Running(self),
      Event::Skip(delta) => {
        self.skip(data, delta);

        Typestate::Running(self)
      },
      Event::Stop => Typestate::Stopped(self.stop()),
    }
  }
}

impl Default for StateOperator<RunningState> {
  fn default() -> Self {
    Self {
      state: PhantomData,
    }
  }
}

impl Display for StateOperator<RunningState> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{STATE_NAME}")
  }
}
