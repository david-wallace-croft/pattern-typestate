use super::ejected::EjectedData;
use super::request::Request;
use super::running::RunningData;
use super::stopped::StoppedData;

#[derive(Debug, PartialEq)]
pub enum Typestate {
  Ejected(EjectedData),
  Running(RunningData),
  Stopped(StoppedData),
}

impl Typestate {
  pub fn transit(
    self,
    event: &Request,
  ) -> Self {
    match self {
      Typestate::Ejected(state_data) => state_data.transit(event),
      Typestate::Running(state_data) => state_data.transit(event),
      Typestate::Stopped(state_data) => state_data.transit(event),
    }
  }
}

impl Default for Typestate {
  fn default() -> Self {
    Typestate::Stopped(StoppedData::new(0))
  }
}
