use super::event::Event;
use super::state_data_a::StateDataA;
use super::state_data_b::StateDataB;
use super::state_data_c::StateDataC;
use super::state_data_d::StateDataD;
use super::state_data_e::StateDataE;

#[derive(Debug, PartialEq)]
pub enum TypeState {
  StateA(StateDataA), // example: uninitialized state
  StateB(StateDataB), // example: initialized state
  StateC(StateDataC), // example: running state
  StateD(StateDataD), // example: suspended state
  StateE(StateDataE), // example: destroyed state
}

impl TypeState {
  pub fn transit(
    self,
    event: &Event,
  ) -> Self {
    match self {
      TypeState::StateA(state_data) => state_data.transit(event),
      TypeState::StateB(state_data) => state_data.transit(event),
      TypeState::StateC(state_data) => state_data.transit(event),
      TypeState::StateD(state_data) => state_data.transit(event),
      TypeState::StateE(state_data) => state_data.transit(event),
    }
  }
}

impl Default for TypeState {
  fn default() -> Self {
    TypeState::StateA(StateDataA {})
  }
}
