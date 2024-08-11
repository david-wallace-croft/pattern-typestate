use super::event::Event;
use super::state_data_b::StateDataB;
use super::type_state::TypeState;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataA {}

impl StateDataA {
  pub fn transit(
    self,
    event: &Event,
  ) -> TypeState {
    match event {
      Event::EventToB => self.transit_to_state_b(),
      _ => TypeState::StateA(self),
    }
  }

  pub fn transit_to_state_b(self) -> TypeState {
    TypeState::StateB(StateDataB::default())
  }
}
