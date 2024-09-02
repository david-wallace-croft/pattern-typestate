use super::super::data::Data;
use super::super::event::Event;
use super::ejected_state::EjectedState;
use super::running_state::RunningState;
use super::state_operator::StateOperator;
use super::stopped_state::StoppedState;

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(StateOperator<EjectedState>),
  Running(StateOperator<RunningState>),
  Stopped(StateOperator<StoppedState>),
}

impl Typestate {
  pub fn get_state_name(&self) -> &'static str {
    match self {
      Typestate::Ejected(state_operator) => state_operator.get_state_name(),
      Typestate::Running(state_operator) => state_operator.get_state_name(),
      Typestate::Stopped(state_operator) => state_operator.get_state_name(),
    }
  }

  pub fn transit(
    self,
    data: &mut Data,
    event: &Event,
  ) -> Self {
    // Outer match on event and inner match on self
    match event {
      Event::Eject => match self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(state_operator) => {
          Typestate::Ejected(state_operator.eject())
        },
      },
      Event::Reset => match &self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(state_operator) => {
          state_operator.reset(data);

          self
        },
      },
      Event::Run => match self {
        Typestate::Ejected(_) | Typestate::Running(_) => self,
        Typestate::Stopped(state_operator) => {
          Typestate::Running(state_operator.run())
        },
      },
      Event::Skip(delta) => match &self {
        Typestate::Ejected(_) | Typestate::Stopped(_) => self,
        Typestate::Running(state_operator) => {
          state_operator.skip(data, *delta);

          self
        },
      },
      Event::Stop => match self {
        Typestate::Ejected(_) | Typestate::Stopped(_) => self,
        Typestate::Running(state_operator) => {
          Typestate::Stopped(state_operator.stop())
        },
      },
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::Stopped(StateOperator::<StoppedState>::default())
  }
}
