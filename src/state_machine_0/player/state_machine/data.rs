#[derive(Debug, Default, PartialEq)]
pub struct Data {
  // Public data fields but the data module is private to module state_machine
  pub position: usize,
}
