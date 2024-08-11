use super::event::Event;
use super::state_data_d::StateDataD;
use super::type_state::TypeState;

#[derive(Debug, PartialEq)]
pub struct StateDataC {}

impl StateDataC {
  pub fn transit(
    self,
    _event: &Event,
  ) -> TypeState {
    TypeState::StateD(StateDataD {})
  }
}
