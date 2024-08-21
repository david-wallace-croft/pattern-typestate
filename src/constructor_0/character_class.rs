#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum CharacterClass {
  #[default]
  None,
  Warrior,
  Wizard,
}
