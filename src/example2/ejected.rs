use super::request::Request;
use super::typestate::Typestate;

#[derive(Debug, PartialEq)]
pub struct EjectedState {
  position: usize,
}

impl EjectedState {
  pub fn get_position(&self) -> usize {
    self.position
  }

  pub fn new(position: usize) -> Self {
    Self {
      position,
    }
  }

  pub fn transit(
    self,
    _request: &Request,
  ) -> Typestate {
    Typestate::Ejected(self)
  }
}
