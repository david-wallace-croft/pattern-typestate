use super::event::Event;
use super::state_data_c::StateDataC;
use super::state_data_e::StateDataE;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct StateDataD {}

impl StateDataD {
  pub fn transit(
    self,
    event: &Event,
  ) -> Typestate {
    match event {
      Event::EventToC => self.transit_to_state_c(),
      Event::EventToE => self.transit_to_state_e(),
      _ => Typestate::StateD(self),
    }
  }

  pub fn transit_to_state_c(self) -> Typestate {
    Typestate::StateC(StateDataC::default())
  }

  pub fn transit_to_state_e(self) -> Typestate {
    Typestate::StateE(StateDataE::default())
  }
}
