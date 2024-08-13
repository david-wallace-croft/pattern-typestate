use super::event::Event;
use super::running::RunningData;
use super::state_data_e::DestroyedData;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct SuspendedData {}

impl SuspendedData {
  pub fn transit(
    self,
    event: &Event,
  ) -> Typestate {
    match event {
      Event::Start => self.transit_to_state_c(),
      Event::Destroy => self.transit_to_state_e(),
      _ => Typestate::Suspended(self),
    }
  }

  pub fn transit_to_state_c(self) -> Typestate {
    Typestate::Running(RunningData::default())
  }

  pub fn transit_to_state_e(self) -> Typestate {
    Typestate::Destroyed(DestroyedData::default())
  }
}
