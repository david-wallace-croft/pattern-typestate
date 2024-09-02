use super::super::event::Event;
use super::data::Data;
use super::ejected_state::EjectedState;
use super::running_state::RunningState;
use super::stopped_state::StoppedState;

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(Data<EjectedState>),
  Running(Data<RunningState>),
  Stopped(Data<StoppedState>),
}

impl Typestate {
  pub fn get_position(&self) -> usize {
    match self {
      Typestate::Ejected(data) => data.get_position(),
      Typestate::Running(data) => data.get_position(),
      Typestate::Stopped(data) => data.get_position(),
    }
  }

  pub fn get_state_name(&self) -> &'static str {
    match self {
      Typestate::Ejected(data) => data.get_state_name(),
      Typestate::Running(data) => data.get_state_name(),
      Typestate::Stopped(data) => data.get_state_name(),
    }
  }

  pub fn transit(
    mut self,
    event: &Event,
  ) -> Self {
    // The outer match is on the event and the inner match on self
    match event {
      Event::Eject => match self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(data) => Typestate::Ejected(data.eject()),
      },
      Event::Reset => match &mut self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(data) => {
          data.reset();

          self
        },
      },
      Event::Run => match self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(data) => Typestate::Running(data.run()),
      },
      Event::Skip(delta) => match &mut self {
        Typestate::Ejected(_) | Typestate::Stopped(_) => self,
        Typestate::Running(data) => {
          data.skip(*delta);

          self
        },
      },
      Event::Stop => match self {
        Typestate::Ejected(_) | Typestate::Stopped(_) => self,
        Typestate::Running(data) => Typestate::Stopped(data.stop()),
      },
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    // The default state starts stopped at position zero
    Typestate::Stopped(Data::<StoppedState>::new(0))
  }
}
