use super::data::Data;
use super::event::Event;
use super::state_machine::StateMachine;

#[derive(Default)]
pub struct Player {
  data: Data,
  state_machine: StateMachine,
}

impl Player {
  pub fn eject(&mut self) {
    self
      .state_machine
      .transit(&mut self.data, Event::Eject);
  }

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

  pub fn reset(&mut self) {
    self
      .state_machine
      .transit(&mut self.data, Event::Reset);
  }

  pub fn run(&mut self) {
    self
      .state_machine
      .transit(&mut self.data, Event::Run);
  }

  pub fn skip(
    &mut self,
    delta: isize,
  ) {
    self
      .state_machine
      .transit(&mut self.data, Event::Skip(delta));
  }

  pub fn stop(&mut self) {
    self
      .state_machine
      .transit(&mut self.data, Event::Stop);
  }
}
