use super::super::event::Event;
use super::super::state_machine::state_trait::StateTrait;
use super::ejected::EjectedState;
use super::running::RunningState;
use super::stopped::StoppedState;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(EjectedState),
  Running(RunningState),
  Stopped(StoppedState),
}

impl Typestate {
  pub fn get_position(&self) -> usize {
    match self {
      Typestate::Ejected(state) => state.get_position(),
      Typestate::Running(state) => state.get_position(),
      Typestate::Stopped(state) => state.get_position(),
    }
  }

  pub fn get_state_name(&self) -> &'static str {
    match self {
      Typestate::Ejected(state) => state.get_state_name(),
      Typestate::Running(state) => state.get_state_name(),
      Typestate::Stopped(state) => state.get_state_name(),
    }
  }

  pub fn transit(
    self,
    event: &Event,
  ) -> Self {
    match self {
      Typestate::Ejected(state) => Typestate::Ejected(state),
      Typestate::Running(mut state) => match event {
        Event::Eject | Event::Reset | Event::Run => Typestate::Running(state),
        Event::Skip(delta) => {
          state.skip(*delta);

          Typestate::Running(state)
        },
        Event::Stop => Typestate::Stopped(state.stop()),
      },
      Typestate::Stopped(mut state) => match event {
        Event::Eject => Typestate::Ejected(state.eject()),
        Event::Reset => {
          state.reset();

          Typestate::Stopped(state)
        },
        Event::Run => Typestate::Running(state.run()),
        Event::Skip(_) | Event::Stop => Typestate::Stopped(state),
      },
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    // The default state starts at position zero
    Typestate::Stopped(StoppedState::new(0))
  }
}

impl Display for Typestate {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    match self {
      Typestate::Ejected(state) => state.fmt(f),
      Typestate::Running(state) => state.fmt(f),
      Typestate::Stopped(state) => state.fmt(f),
    }
  }
}
