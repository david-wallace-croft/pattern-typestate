use super::super::data::Data;
use super::super::event::Event;
use super::super::state_machine::ejected::EjectedState;
use super::super::state_machine::running::RunningState;
use super::super::state_machine::state_operator::StateOperator;
use super::super::state_machine::state_trait::StateTrait;
use super::super::state_machine::typestate::Typestate;
use std::fmt::{Display, Formatter, Result};
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
  fn eject(self) -> StateOperator<EjectedState> {
    StateOperator::<EjectedState>::default()
  }

  fn reset(
    &self,
    data: &mut Data,
  ) {
    data.position = 0;
  }

  fn run(self) -> StateOperator<RunningState> {
    StateOperator::<RunningState>::default()
  }

  pub fn transit(
    self,
    data: &mut Data,
    event: Event,
  ) -> Typestate {
    match event {
      Event::Eject => Typestate::Ejected(self.eject()),
      Event::Reset => {
        self.reset(data);

        Typestate::Stopped(self)
      },
      Event::Run => Typestate::Running(self.run()),
      Event::Skip(_) | Event::Stop => Typestate::Stopped(self),
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
  ) -> Result {
    write!(f, "{STATE_NAME}")
  }
}
