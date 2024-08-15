use crate::strict_builder_1::player_character::{
  Armor, PlayerCharacter, Spell, WarriorWeapon, WizardWeapon,
};

pub mod player_character;

// TODO: clippy

pub fn example() {
  let _player_character: PlayerCharacter = PlayerCharacter::warrior_builder()
    .armor(Armor::Chainmail)
    .warrior_weapon(WarriorWeapon::LongSword);

  let _player_character: PlayerCharacter = PlayerCharacter::wizard_builder()
    .spell(Spell::Invisibility)
    .wizard_weapon(WizardWeapon::Staff);

  // Will not compile; warriors cannot use spells
  // let _player_character: PlayerCharacter = PlayerCharacter::warrior_builder()
  //   .spell(Spell::Invisibility)
  //   .warrior_weapon(WarriorWeapon::LongSword);

  // Will not compile; warriors are not restricted to the wizard weapon subset
  // let _player_character: PlayerCharacter = PlayerCharacter::warrior_builder()
  //   .armor(Armor::Chainmail)
  //   .wizard_weapon(WizardWeapon::Staff);

  // Will not compile; wizards cannot wear armor
  // let _player_character: PlayerCharacter = PlayerCharacter::wizard_builder()
  //   .armor(Armor::Chainmail)
  //   .wizard_weapon(WizardWeapon::Staff);

  // Will not compile; wizards cannot use warrior weapons
  // let _player_character: PlayerCharacter = PlayerCharacter::wizard_builder()
  //   .spell(Spell::Invisibility)
  //   .warrior_weapon(WarriorWeapon::LongSword);
}
