use super::armor::Armor;
use super::character_class::CharacterClass;
use super::spell::Spell;
use super::weapon::Weapon;

// The constructor submodule provides a ConstructorCreator trait implementation
mod constructor;

// The private fields can only be set in the constructor submodule
pub struct PlayerCharacter {
  armor: Armor,
  character_class: CharacterClass,
  health: isize,
  spell: Spell,
  weapon: Weapon,
  wealth: f64,
  wisdom: usize,
}

// Provides field accessor (getter) methods but no mutator (setter) methods
impl PlayerCharacter {
  pub fn armor(&self) -> Armor {
    self.armor
  }

  pub fn character_class(&self) -> CharacterClass {
    self.character_class
  }

  pub fn health(&self) -> isize {
    self.health
  }

  pub fn spell(&self) -> Spell {
    self.spell
  }

  pub fn weapon(&self) -> Weapon {
    self.weapon
  }

  pub fn wealth(&self) -> f64 {
    self.wealth
  }

  pub fn wisdom(&self) -> usize {
    self.wisdom
  }
}
