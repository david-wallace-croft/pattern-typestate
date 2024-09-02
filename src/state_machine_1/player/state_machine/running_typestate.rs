use super::super::data::Data;
use super::stopped_typestate::StoppedTypestate;
use super::typestate_trait::TypestateTrait;

const STATE_NAME: &str = "RUNNING";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunningTypestate;

impl RunningTypestate {
  pub fn skip(
    &self,
    data: &mut Data,
    delta: isize,
  ) {
    data.position = data
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> StoppedTypestate {
    StoppedTypestate
  }
}

impl TypestateTrait for RunningTypestate {
  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
