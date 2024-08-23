/// The superset of all weapons
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum Weapon {
  Dagger,
  Mace,
  #[default]
  None,
  LongSword,
  Staff,
}
