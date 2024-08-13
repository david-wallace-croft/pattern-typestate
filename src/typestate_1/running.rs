use super::event::Event;
use super::state_data_e::DestroyedData;
use super::suspended::SuspendedData;
use super::typestate::Typestate;

#[derive(Debug, Default, PartialEq)]
pub struct RunningData {
  value: usize,
}

impl RunningData {
  pub fn some_mutator_method_unique_to_state_c(&mut self) {
    self.value *= 2;
  }

  pub fn transit(
    self,
    event: &Event,
  ) -> Typestate {
    match event {
      Event::Stop => self.stop(),
      Event::Destroy => self.destroy(),
      _ => Typestate::Running(self),
    }
  }

  pub fn stop(self) -> Typestate {
    Typestate::Suspended(SuspendedData::default())
  }

  pub fn destroy(self) -> Typestate {
    Typestate::Destroyed(DestroyedData::default())
  }
}
