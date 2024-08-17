use super::armor::Armor;
use super::character_class::CharacterClass;
use super::spell::Spell;
use super::weapon::Weapon;
use fluent_constructor::FluentConstructorPlayerCharacter;

mod fluent_constructor;

pub struct PlayerCharacter {
  armor: Armor,
  character_class: CharacterClass,
  health: isize,
  spell: Spell,
  weapon: Weapon,
  wealth: f64,
  wisdom: usize,
}

impl PlayerCharacter {
  pub fn armor(&self) -> Armor {
    self.armor
  }

  pub fn character_class(&self) -> CharacterClass {
    self.character_class
  }

  pub fn constructor() -> FluentConstructorPlayerCharacter {
    let player_character = PlayerCharacter {
      armor: Armor::None,
      character_class: CharacterClass::None,
      health: 0,
      spell: Spell::None,
      weapon: Weapon::None,
      wealth: 0.,
      wisdom: 0,
    };

    FluentConstructorPlayerCharacter::new(player_character)
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
