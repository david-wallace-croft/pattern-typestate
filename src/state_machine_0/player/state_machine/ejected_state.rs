use super::state_trait::StateTrait;

const STATE_NAME: &str = "EJECTED";

#[derive(Debug, PartialEq)]
pub struct EjectedState;

impl StateTrait for EjectedState {
  fn get_state_name() -> &'static str {
    STATE_NAME
  }
}
