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
    self,
    event: &Event,
  ) -> Self {
    // Outer match on self and inner match on event
    match self {
      Typestate::Ejected(_) => self,
      Typestate::Running(mut data) => match event {
        Event::Eject | Event::Reset | Event::Run => Typestate::Running(data),
        Event::Skip(delta) => {
          data.skip(*delta);

          Typestate::Running(data)
        },
        Event::Stop => Typestate::Stopped(data.stop()),
      },
      Typestate::Stopped(mut data) => match event {
        Event::Eject => Typestate::Ejected(data.eject()),
        Event::Reset => {
          data.reset();

          Typestate::Stopped(data)
        },
        Event::Run => Typestate::Running(data.run()),
        Event::Skip(_) | Event::Stop => Typestate::Stopped(data),
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
