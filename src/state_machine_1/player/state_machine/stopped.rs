use super::super::data::Data;
use super::super::event::Event;
use super::super::state_machine::ejected::EjectedState;
use super::super::state_machine::running::RunningState;
use super::super::state_machine::state_operator::StateOperator;
use super::super::state_machine::state_trait::StateTrait;
use super::super::state_machine::typestate::Typestate;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

const STATE_NAME: &str = "STOPPED";

#[derive(Debug, PartialEq)]
pub struct StoppedState;

impl StateTrait for StoppedState {
  fn get_state_name() -> &'static str {
    STATE_NAME
  }
}

impl StateOperator<StoppedState> {
  fn eject(&self) -> StateOperator<EjectedState> {
    StateOperator::<EjectedState>::new()
  }

  fn reset(
    &self,
    data: &mut Data,
  ) -> StateOperator<StoppedState> {
    data.position = 0;

    StateOperator::<StoppedState>::default()
  }

  fn run(&self) -> StateOperator<RunningState> {
    StateOperator::<RunningState>::new()
  }

  pub fn transit(
    self,
    data: &mut Data,
    event: Event,
  ) -> Typestate {
    match event {
      Event::Eject => Typestate::Ejected(self.eject()),
      Event::Reset => Typestate::Stopped(self.reset(data)),
      Event::Run => Typestate::Running(self.run()),
      Event::Skip(_) | Event::Stop => {
        Typestate::Stopped(StateOperator::<StoppedState>::default())
      },
    }
  }
}

impl Default for StateOperator<StoppedState> {
  fn default() -> Self {
    StateOperator {
      state: PhantomData,
    }
  }
}

impl Display for StateOperator<StoppedState> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{STATE_NAME}")
  }
}
