use super::event::Event;
use super::state_data_c::StateDataC;
use super::state_data_e::StateDataE;
use super::type_state::TypeState;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataD {}

impl StateDataD {
  pub fn transit(
    self,
    event: &Event,
  ) -> TypeState {
    match event {
      Event::EventToC => self.transit_to_state_c(),
      Event::EventToE => self.transit_to_state_e(),
      _ => TypeState::StateD(self),
    }
  }

  pub fn transit_to_state_c(self) -> TypeState {
    TypeState::StateC(StateDataC::default())
  }

  pub fn transit_to_state_e(self) -> TypeState {
    TypeState::StateE(StateDataE::default())
  }
}
