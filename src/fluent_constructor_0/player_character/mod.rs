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
}