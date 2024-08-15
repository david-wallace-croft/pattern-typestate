use crate::strict_builder_1::player_character::{
  Armor, PlayerCharacter, Spell, Weapon, WizardWeapon,
};

pub mod player_character;

pub fn example() {
  let _player_character: PlayerCharacter = PlayerCharacter::builder()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail);

  let _player_character: PlayerCharacter = PlayerCharacter::builder()
    .wizard()
    .weapon(WizardWeapon::Staff)
    .spell(Spell::Invisibility);

  // Does not compile; the character class must be chosen first
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .weapon(WarriorWeapon::LongSword)
  //   .warrior()
  //   .armor(Armor::Chainmail);

  // Does not compile; warriors are not restricted to the wizard weapon subset
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .warrior()
  //   .weapon(WizardWeapon::Staff)
  //   .armor(Armor::Chainmail);

  // Does not compile; wizards are restricted to the wizard weapon subset
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .wizard()
  //   .weapon(Weapon::LongSword)
  //   .spell(Spell::Invisibility);

  // Does not compile; warriors cannot cast a spell
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .warrior()
  //   .weapon(Weapon::LongSword)
  //   .spell(Spell::Invisibility);

  // Does not compile; wizards cannot wear armor
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .wizard()
  //   .weapon(WizardWeapon::Staff)
  //   .armor(Armor::Chainmail);
}
