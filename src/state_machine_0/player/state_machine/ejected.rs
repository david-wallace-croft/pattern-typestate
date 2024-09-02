use super::state_trait::StateTrait;

const STATE_NAME: &str = "EJECTED";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EjectedState;

impl StateTrait for EjectedState {
  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
