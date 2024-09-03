//==============================================================================
//! An example of using the Typestate pattern to implement a state machine.
//!
//! - In this example, the StateMachine operates on Data from outside itself.
//! - The Data is passed into the StateMachine with each Event.
//! - Since the Typestate structs do not need to hold Data, they are simple.
//! - Since there is no shared code, each state has its own Typestate struct.
//! - A main Typestate struct with a state discriminant generic is not needed.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-02
//! - Updated: 2024-09-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

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
  // No get_position() method since the Data is stored outside the StateMachine.

  pub fn get_state_name(&self) -> &'static str {
    match self {
      StateMachine::Ejected(_) => EjectedTypestate::get_state_name(),
      StateMachine::Running(_) => RunningTypestate::get_state_name(),
      StateMachine::Stopped(_) => StoppedTypestate::get_state_name(),
    }
  }

  pub fn transit(
    self,
    data: &mut Data,
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
      Event::Reset => match &self {
        StateMachine::Ejected(_) => self,
        StateMachine::Running(_) => self,
        StateMachine::Stopped(typestate) => {
          typestate.reset(data);

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
      Event::Skip(delta) => match &self {
        StateMachine::Ejected(_) => self,
        StateMachine::Stopped(_) => self,
        StateMachine::Running(typestate) => {
          typestate.skip(data, *delta);

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
    StateMachine::Stopped(StoppedTypestate)
  }
}
