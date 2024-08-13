use super::event::Event;
use super::state_data_d::StateDataD;
use super::state_data_e::StateDataE;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataC {
  value: usize,
}

impl StateDataC {
  pub fn some_mutator_method_unique_to_state_c(&mut self) {
    self.value *= 2;
  }

  pub fn transit(
    self,
    event: &Event,
  ) -> Typestate {
    match event {
      Event::EventToD => self.transit_to_state_d(),
      Event::EventToE => self.transit_to_state_e(),
      _ => Typestate::StateC(self),
    }
  }

  pub fn transit_to_state_d(self) -> Typestate {
    Typestate::StateD(StateDataD::default())
  }

  pub fn transit_to_state_e(self) -> Typestate {
    Typestate::StateE(StateDataE::default())
  }
}