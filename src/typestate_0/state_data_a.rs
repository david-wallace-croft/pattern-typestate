use super::event::Event;
use super::state_data_b::StateDataB;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataA {}

impl StateDataA {
  pub fn transit(
    self,
    event: &Event,
  ) -> Typestate {
    match event {
      Event::EventToB => self.transit_to_state_b(),
      _ => Typestate::StateA(self),
    }
  }

  pub fn transit_to_state_b(self) -> Typestate {
    Typestate::StateB(StateDataB::default())
  }
}
