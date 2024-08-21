#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum Armor {
  Chainmail,
  #[default]
  None,
  Plate,
}
