use super::state_trait::StateTrait;

const STATE_NAME: &str = "RUNNING";

#[derive(Debug, PartialEq)]
pub struct RunningState;

impl StateTrait for RunningState {
  fn get_state_name() -> &'static str {
    STATE_NAME
  }
}
