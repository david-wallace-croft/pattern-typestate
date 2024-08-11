use super::event::Event;
use super::initialized::InitializedData;
use super::running::RunningData;
use super::state_data_e::DestroyedData;
use super::suspended::SuspendedData;
use super::uninitialized::UninitializedData;

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Uninitialized(UninitializedData),
  Initialized(InitializedData),
  Running(RunningData),
  Suspended(SuspendedData),
  Destroyed(DestroyedData),
}

impl Typestate {
  pub fn transit(
    self,
    event: &Event,
  ) -> Self {
    match self {
      Typestate::Uninitialized(state_data) => state_data.transit(event),
      Typestate::Initialized(state_data) => state_data.transit(event),
      Typestate::Running(state_data) => state_data.transit(event),
      Typestate::Suspended(state_data) => state_data.transit(event),
      Typestate::Destroyed(state_data) => state_data.transit(event),
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::Uninitialized(UninitializedData {})
  }
}
