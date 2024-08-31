use crate::state_machine_2::data::Data;
use crate::state_machine_2::event::Event;
use crate::state_machine_2::state_machine::ejected::EjectedState;
use crate::state_machine_2::state_machine::running::RunningState;
use crate::state_machine_2::state_machine::state_operator::StateOperator;
use crate::state_machine_2::state_machine::state_trait::StateTrait;
use crate::state_machine_2::state_machine::typestate::Typestate;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct StoppedState;

impl StateTrait for StoppedState {}

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
    write!(f, "STOPPED")
  }
}
