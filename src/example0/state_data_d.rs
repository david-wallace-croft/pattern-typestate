use super::event::Event;
use super::state_data_e::StateDataE;
use super::type_state::TypeState;

#[derive(Debug, PartialEq)]
pub struct StateDataD {}

impl StateDataD {
  pub fn transit(
    self,
    _event: &Event,
  ) -> TypeState {
    TypeState::StateE(StateDataE {})
  }
}
