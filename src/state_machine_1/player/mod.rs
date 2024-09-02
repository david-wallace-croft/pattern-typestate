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
  // accessor methods

  pub fn get_position(&self) -> usize {
    self
      .data
      .position
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
    self.state_machine = self
      .state_machine
      .transit(&mut self.data, event);
  }
}
