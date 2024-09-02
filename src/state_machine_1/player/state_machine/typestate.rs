use super::super::data::Data;
use super::super::event::Event;
use super::ejected_state::EjectedState;
use super::running_state::RunningState;
use super::state_trait::StateTrait;
use super::stopped_state::StoppedState;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Typestate {
  Ejected(EjectedState),
  Running(RunningState),
  Stopped(StoppedState),
}

impl Typestate {
  pub fn get_state_name(&self) -> &'static str {
    match self {
      Typestate::Ejected(state) => state.get_state_name(),
      Typestate::Running(state) => state.get_state_name(),
      Typestate::Stopped(state) => state.get_state_name(),
    }
  }

  pub fn transit(
    self,
    data: &mut Data,
    event: &Event,
  ) -> Self {
    // The outer match is on the event and the inner match on self
    match event {
      Event::Eject => match self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(state) => Typestate::Ejected(state.eject()),
      },
      Event::Reset => match &self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(state) => {
          state.reset(data);

          self
        },
      },
      Event::Run => match self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(state) => Typestate::Running(state.run()),
      },
      Event::Skip(delta) => match &self {
        Typestate::Ejected(_) | Typestate::Stopped(_) => self,
        Typestate::Running(state) => {
          state.skip(data, *delta);

          self
        },
      },
      Event::Stop => match self {
        Typestate::Ejected(_) | Typestate::Stopped(_) => self,
        Typestate::Running(state) => Typestate::Stopped(state.stop()),
      },
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::Stopped(StoppedState)
  }
}
