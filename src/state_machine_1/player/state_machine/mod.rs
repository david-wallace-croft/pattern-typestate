use self::ejected_typestate::EjectedTypestate;
use self::running_typestate::RunningTypestate;
use self::stopped_typestate::StoppedTypestate;
use self::typestate_trait::TypestateTrait;
use super::data::Data;
use super::event::Event;

// The StateMachine submodules are private
mod ejected_typestate;
mod running_typestate;
mod stopped_typestate;
mod typestate_trait;

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StateMachine {
  Ejected(EjectedTypestate),
  Running(RunningTypestate),
  Stopped(StoppedTypestate),
}

impl StateMachine {
  pub fn get_state_name(&self) -> &'static str {
    match self {
      StateMachine::Ejected(typestate) => typestate.get_state_name(),
      StateMachine::Running(typestate) => typestate.get_state_name(),
      StateMachine::Stopped(typestate) => typestate.get_state_name(),
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
        StateMachine::Ejected(_) | StateMachine::Running(_) => self,
        StateMachine::Stopped(typestate) => {
          StateMachine::Ejected(typestate.eject())
        },
      },
      Event::Reset => match &self {
        StateMachine::Ejected(_) | StateMachine::Running(_) => self,
        StateMachine::Stopped(typestate) => {
          typestate.reset(data);

          self
        },
      },
      Event::Run => match self {
        StateMachine::Ejected(_) | StateMachine::Running(_) => self,
        StateMachine::Stopped(typestate) => {
          StateMachine::Running(typestate.run())
        },
      },
      Event::Skip(delta) => match &self {
        StateMachine::Ejected(_) | StateMachine::Stopped(_) => self,
        StateMachine::Running(typestate) => {
          typestate.skip(data, *delta);

          self
        },
      },
      Event::Stop => match self {
        StateMachine::Ejected(_) | StateMachine::Stopped(_) => self,
        StateMachine::Running(typestate) => {
          StateMachine::Stopped(typestate.stop())
        },
      },
    }
  }
}

impl Default for StateMachine {
  fn default() -> Self {
    StateMachine::Stopped(StoppedTypestate)
  }
}
