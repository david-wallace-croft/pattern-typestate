use super::state_trait::StateTrait;

const STATE_NAME: &str = "STOPPED";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoppedState;

impl StateTrait for StoppedState {
  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
