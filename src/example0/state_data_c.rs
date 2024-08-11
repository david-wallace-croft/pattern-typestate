use super::event::Event;
use super::state_data_d::StateDataD;
use super::state_data_e::StateDataE;
use super::type_state::TypeState;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataC {}

impl StateDataC {
  pub fn transit(
    self,
    event: &Event,
  ) -> TypeState {
    match event {
      Event::EventToD => self.transit_to_state_d(),
      Event::EventToE => self.transit_to_state_e(),
      _ => TypeState::StateC(self),
    }
  }

  pub fn transit_to_state_d(self) -> TypeState {
    TypeState::StateD(StateDataD::default())
  }

  pub fn transit_to_state_e(self) -> TypeState {
    TypeState::StateE(StateDataE::default())
  }
}
