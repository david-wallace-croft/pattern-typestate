use super::event::Event;
use super::typestate::Typestate;
use crate::example0::state_data_c::StateDataC;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataB {
  pub count: usize,
}

impl StateDataB {
  pub fn some_accessor_method_unique_to_state_b(&self) -> usize {
    self.count
  }

  pub fn some_mutator_method_unique_to_state_b(&mut self) {
    self.count += 1;
  }

  pub fn transit(
    self,
    event: &Event,
  ) -> Typestate {
    match event {
      Event::EventToC => self.transit_to_state_c(),
      _ => Typestate::StateB(self),
    }
  }

  pub fn transit_to_state_c(self) -> Typestate {
    Typestate::StateC(StateDataC::default())
  }
}
