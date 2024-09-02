use super::state_trait::StateTrait;

const STATE_NAME: &str = "EJECTED";

#[derive(Debug, PartialEq)]
pub struct EjectedState;

impl EjectedState {
  // No state transition methods for the ejected state
}

impl StateTrait for EjectedState {
  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
