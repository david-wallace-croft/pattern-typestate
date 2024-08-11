use super::event::Event;
use super::type_state::TypeState;
use crate::example0::state_data_c::StateDataC;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataB {
  pub count: usize,
}

impl StateDataB {
  pub fn some_function_unique_to_state_b(&mut self) {
    self.count += 1;
  }

  pub fn transit(
    self,
    event: &Event,
  ) -> TypeState {
    match event {
      Event::EventToC => self.transit_to_state_c(),
      _ => TypeState::StateB(self),
    }
  }

  pub fn transit_to_state_c(self) -> TypeState {
    TypeState::StateC(StateDataC::default())
  }
}
