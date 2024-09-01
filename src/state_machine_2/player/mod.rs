use self::data::Data;
use self::event::Event;
use self::state_machine::StateMachine;

// The Player submodules are private
mod data;
mod event;
mod state_machine;

#[derive(Default)]
pub struct Player {
  data: Data,
  state_machine: StateMachine,
}

impl Player {
  // access methods

  pub fn get_position(&self) -> usize {
    self
      .data
      .position
  }

  pub fn get_state(&self) -> String {
    self
      .state_machine
      .get_state()
  }

  // mutator methods

  // TODO: Should mutator methods return Result?

  pub fn request_eject(&mut self) {
    self
      .state_machine
      .transit(&mut self.data, Event::Eject);
  }

  pub fn request_reset(&mut self) {
    self
      .state_machine
      .transit(&mut self.data, Event::Reset);
  }

  pub fn request_run(&mut self) {
    self
      .state_machine
      .transit(&mut self.data, Event::Run);
  }

  pub fn request_skip(
    &mut self,
    delta: isize,
  ) {
    self
      .state_machine
      .transit(&mut self.data, Event::Skip(delta));
  }

  pub fn request_stop(&mut self) {
    self
      .state_machine
      .transit(&mut self.data, Event::Stop);
  }
}
