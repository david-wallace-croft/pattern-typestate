//==============================================================================
//! A Player sends Events to its internal StateMachine.
//!
//! In this example, the Data for the Player is inside the StateMachine.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-02
//! - Updated: 2024-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use self::event::Event;
use self::state_machine::StateMachine;
use std::mem;

// The Player submodules are private
mod event;
mod state_machine;

#[derive(Default)]
pub struct Player {
  state_machine: StateMachine,
}

impl Player {
  // accessor methods

  pub fn get_position(&self) -> usize {
    self
      .state_machine
      .get_position()
  }

  pub fn get_state_name(&self) -> &'static str {
    self
      .state_machine
      .get_state_name()
  }

  // mutator methods

  pub fn press_eject(&mut self) {
    self.transit(&Event::Eject);
  }

  pub fn press_reset(&mut self) {
    self.transit(&Event::Reset);
  }

  pub fn press_run(&mut self) {
    self.transit(&Event::Run);
  }

  pub fn press_skip(
    &mut self,
    delta: isize,
  ) {
    self.transit(&Event::Skip(delta));
  }

  pub fn press_stop(&mut self) {
    self.transit(&Event::Stop);
  }

  // private methods

  fn transit(
    &mut self,
    event: &Event,
  ) {
    // Doing this directly will not compile; cannot move and cannot copy because
    // StateMachine uses Typestate which does not implement Copy.
    //
    // self.state_machine = self.state_machine.transit(event);

    let state_machine: StateMachine = mem::take(&mut self.state_machine);

    self.state_machine = state_machine.transit(event);
  }
}
