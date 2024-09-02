use self::event::Event;
use self::state_machine::StateMachine;

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
    self
      .state_machine
      .transit(&Event::Eject);
  }

  pub fn press_reset(&mut self) {
    self
      .state_machine
      .transit(&Event::Reset);
  }

  pub fn press_run(&mut self) {
    self
      .state_machine
      .transit(&Event::Run);
  }

  pub fn press_skip(
    &mut self,
    delta: isize,
  ) {
    self
      .state_machine
      .transit(&Event::Skip(delta));
  }

  pub fn press_stop(&mut self) {
    self
      .state_machine
      .transit(&Event::Stop);
  }
}
