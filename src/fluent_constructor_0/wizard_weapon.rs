use super::weapon::Weapon;

/// The subset of weapons available to wizards
pub enum WizardWeapon {
  Dagger,
  None,
  Staff,
}

impl From<WizardWeapon> for Weapon {
  fn from(value: WizardWeapon) -> Self {
    match value {
      WizardWeapon::Dagger => Weapon::Dagger,
      WizardWeapon::None => Weapon::None,
      WizardWeapon::Staff => Weapon::Staff,
    }
  }
}
