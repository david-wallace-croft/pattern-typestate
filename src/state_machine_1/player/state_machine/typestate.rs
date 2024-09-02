use super::super::data::Data;
use super::super::event::Event;
use super::ejected::EjectedState;
use super::running::RunningState;
use super::state_operator::StateOperator;
use super::stopped::StoppedState;

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
    // TODO: outer match on Event
    match self {
      Typestate::Ejected(_) => self,
      Typestate::Running(state_operator) => match event {
        Event::Eject | Event::Reset | Event::Run => {
          Typestate::Running(state_operator)
        },
        Event::Skip(delta) => {
          state_operator.skip(data, *delta);

          Typestate::Running(state_operator)
        },
        Event::Stop => Typestate::Stopped(state_operator.stop()),
      },
      Typestate::Stopped(state_operator) => match event {
        Event::Eject => Typestate::Ejected(state_operator.eject()),
        Event::Reset => {
          state_operator.reset(data);

          Typestate::Stopped(state_operator)
        },
        Event::Run => Typestate::Running(state_operator.run()),
        Event::Skip(_) | Event::Stop => Typestate::Stopped(state_operator),
      },
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::Stopped(StateOperator::<StoppedState>::default())
  }
}
