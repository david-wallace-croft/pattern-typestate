use super::event::Event;
use super::type_state::TypeState;

#[derive(Debug, PartialEq)]
pub struct StateDataE {}

impl StateDataE {
  pub fn transit(
    self,
    _event: &Event,
  ) -> TypeState {
    TypeState::StateE(self)
  }
}
