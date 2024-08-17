use super::armor::Armor;
use super::character_class::CharacterClass;
use super::spell::Spell;
use super::weapon::Weapon;
use strict_builder::StrictBuilderPlayerCharacter;

mod strict_builder;

pub struct PlayerCharacter {
  armor: Armor,
  character_class: CharacterClass,
  spell: Spell,
  weapon: Weapon,
}

impl PlayerCharacter {
  pub fn builder() -> StrictBuilderPlayerCharacter {
    let player_character = PlayerCharacter {
      armor: Armor::None,
      character_class: CharacterClass::None,
      spell: Spell::None,
      weapon: Weapon::None,
    };

    StrictBuilderPlayerCharacter::new(player_character)
  }
}
