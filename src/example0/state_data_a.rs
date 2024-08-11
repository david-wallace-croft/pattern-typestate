use super::event::Event;
use super::state_data_b::StateDataB;
use super::type_state::TypeState;

#[derive(Debug, PartialEq)]
pub struct StateDataA {}

impl StateDataA {
  pub fn transit(
    self,
    _event: &Event,
  ) -> TypeState {
    TypeState::StateB(StateDataB {})
  }
}
