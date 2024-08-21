#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum Spell {
  Invisibility,
  #[default]
  None,
  Sleep,
}
