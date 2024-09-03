use super::state_trait::StateTrait;

const STATE_NAME: &str = "STOPPED";

#[derive(Debug, PartialEq)]
pub struct StoppedState;

impl StateTrait for StoppedState {
  fn get_state_name() -> &'static str {
    STATE_NAME
  }
}
