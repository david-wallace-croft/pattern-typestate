use super::event::Event;
use super::state_data_a::StateDataA;
use super::state_data_b::StateDataB;
use super::state_data_c::StateDataC;
use super::state_data_d::StateDataD;
use super::state_data_e::StateDataE;

#[derive(Debug, PartialEq)]
pub enum Typestate {
  StateA(StateDataA), // example: uninitialized state
  StateB(StateDataB), // example: initialized state
  StateC(StateDataC), // example: running state
  StateD(StateDataD), // example: suspended state
  StateE(StateDataE), // example: destroyed state
}

impl Typestate {
  pub fn transit(
    self,
    event: &Event,
  ) -> Self {
    match self {
      Typestate::StateA(state_data) => state_data.transit(event),
      Typestate::StateB(state_data) => state_data.transit(event),
      Typestate::StateC(state_data) => state_data.transit(event),
      Typestate::StateD(state_data) => state_data.transit(event),
      Typestate::StateE(state_data) => state_data.transit(event),
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::StateA(StateDataA {})
  }
}
