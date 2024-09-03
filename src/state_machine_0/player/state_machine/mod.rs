use self::ejected_state::EjectedState;
use self::running_state::RunningState;
use self::state_trait::StateTrait;
use self::stopped_state::StoppedState;
use self::typestate::Typestate;
use super::event::Event;

// The StateMachine submodules are private
mod ejected_state;
mod running_state;
mod state_trait;
mod stopped_state;
mod typestate;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub enum StateMachine {
  Ejected(Typestate<EjectedState>),
  Running(Typestate<RunningState>),
  Stopped(Typestate<StoppedState>),
}

impl StateMachine {
  pub fn get_position(&self) -> usize {
    match self {
      StateMachine::Ejected(typestate) => typestate.get_position(),
      StateMachine::Running(typestate) => typestate.get_position(),
      StateMachine::Stopped(typestate) => typestate.get_position(),
    }
  }

  pub fn get_state_name(&self) -> &'static str {
    match self {
      StateMachine::Ejected(_) => EjectedState::get_state_name(),
      StateMachine::Running(_) => RunningState::get_state_name(),
      StateMachine::Stopped(_) => StoppedState::get_state_name(),
    }
  }

  pub fn transit(
    mut self,
    event: &Event,
  ) -> Self {
    // The outer match is on the event and the inner match is on self
    match event {
      Event::Eject => match self {
        StateMachine::Ejected(_) => self,
        StateMachine::Running(_) => self,
        StateMachine::Stopped(typestate) => {
          StateMachine::Ejected(typestate.eject())
        },
      },
      Event::Reset => match &mut self {
        StateMachine::Ejected(_) => self,
        StateMachine::Running(_) => self,
        StateMachine::Stopped(typestate) => {
          typestate.reset();

          self
        },
      },
      Event::Run => match self {
        StateMachine::Ejected(_) => self,
        StateMachine::Running(_) => self,
        StateMachine::Stopped(typestate) => {
          StateMachine::Running(typestate.run())
        },
      },
      Event::Skip(delta) => match &mut self {
        StateMachine::Ejected(_) => self,
        StateMachine::Stopped(_) => self,
        StateMachine::Running(typestate) => {
          typestate.skip(*delta);

          self
        },
      },
      Event::Stop => match self {
        StateMachine::Ejected(_) => self,
        StateMachine::Stopped(_) => self,
        StateMachine::Running(typestate) => {
          StateMachine::Stopped(typestate.stop())
        },
      },
    }
  }
}

impl Default for StateMachine {
  fn default() -> Self {
    // The default state starts stopped at position zero
    StateMachine::Stopped(Typestate::<StoppedState>::new(0))
  }
}
