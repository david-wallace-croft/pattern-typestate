use super::state_trait::StateTrait;

const STATE_NAME: &str = "RUNNING";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunningState;

impl StateTrait for RunningState {
  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
