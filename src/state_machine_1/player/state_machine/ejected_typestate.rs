use super::typestate_trait::TypestateTrait;

const STATE_NAME: &str = "EJECTED";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EjectedTypestate;

impl EjectedTypestate {
  // No state transition methods for the ejected state
}

impl TypestateTrait for EjectedTypestate {
  fn get_state_name(&self) -> &'static str {
    STATE_NAME
  }
}
