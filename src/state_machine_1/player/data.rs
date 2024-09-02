#[derive(Debug, Default, PartialEq)]
pub struct Data {
  // Data fields are public but the data module is private to the player module
  pub position: usize,
}
