use super::event::Event;
use super::state_data_c::StateDataC;
use super::type_state::TypeState;

#[derive(Debug, PartialEq)]
pub struct StateDataB {}

impl StateDataB {
  pub fn transit(
    self,
    _event: &Event,
  ) -> TypeState {
    TypeState::StateC(StateDataC {})
  }
}
