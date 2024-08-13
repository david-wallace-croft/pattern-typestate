use super::event::Event;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataE {}

impl StateDataE {
  pub fn transit(
    self,
    _event: &Event,
  ) -> Typestate {
    Typestate::StateE(self)
  }
}
