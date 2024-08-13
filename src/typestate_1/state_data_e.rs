use super::event::Event;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct DestroyedData {}

impl DestroyedData {
  pub fn transit(
    self,
    _event: &Event,
  ) -> Typestate {
    Typestate::Destroyed(self)
  }
}
