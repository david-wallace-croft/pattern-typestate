use super::event::Event;
use super::running::RunningData;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct InitializedData {
  pub count: usize,
}

impl InitializedData {
  pub fn some_accessor_method_unique_to_state_b(&self) -> usize {
    self.count
  }

  pub fn some_mutator_method_unique_to_state_b(&mut self) {
    self.count += 1;
  }

  pub fn transit(
    self,
    event: &Event,
  ) -> Typestate {
    match event {
      Event::Start => self.start(),
      _ => Typestate::Initialized(self),
    }
  }

  pub fn start(self) -> Typestate {
    Typestate::Running(RunningData::default())
  }
}
