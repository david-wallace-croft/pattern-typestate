use super::event::Event;
use super::initialized::InitializedData;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct UninitializedData {}

impl UninitializedData {
  pub fn transit(
    self,
    event: &Event,
  ) -> Typestate {
    match event {
      Event::Initialize => self.initialize(),
      _ => Typestate::Uninitialized(self),
    }
  }

  pub fn initialize(self) -> Typestate {
    Typestate::Initialized(InitializedData::default())
  }
}
