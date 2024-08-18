#![allow(dead_code)]

use self::armor::Armor;
use self::constructor_creator::ConstructorCreator;
use self::player_character::PlayerCharacter;
use self::spell::Spell;
use self::weapon::Weapon;
use self::wizard_weapon::WizardWeapon;

mod armor;
mod character_class;
mod constructor_creator;
mod player_character;
mod spell;
mod weapon;
mod wizard_weapon;

#[cfg(test)]
mod test;

pub fn example() {
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health(10)
    .wealth(10.)
    .wisdom(10);

  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .wizard()
    .weapon(WizardWeapon::Staff)
    .spell(Spell::Invisibility)
    .health(4)
    .wealth(20.)
    .wisdom(15);

  // Some fields such as health have character class-specific default values
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health_default()
    .wealth(10.)
    .wisdom(10);

  // Provide values only where required and use default values for the rest
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .default();

  // TODO: update examples for new fields
  // TODO: reuse the examples from strict_builder_0 then delete

  // ===========================================================================
  // Does not compile; cannot use a structure literal because fields are private
  //
  // let _player_character: PlayerCharacter = PlayerCharacter {
  //   armor: Armor::Chainmail,
  //   character_class: CharacterClass::Wizard,
  //   spell: Spell::Invisibility,
  //   weapon: Weapon::LongSword,
  // };
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; PlayerCharacter does not implement the Default trait
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::default();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; PlayerCharacter does not have a static constructor
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::new(
  //   Armor::Chainmail,
  //   CharacterClass::Wizard,
  //   Spell::Invisibility,
  //   Weapon::LongSword,
  // );
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; the character class must be chosen first
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .weapon(Weapon::LongSword)
  //   .wizard()
  //   .spell(Spell::Invisibility);
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; warriors are not restricted to the wizard weapon subset
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .warrior()
  //   .weapon(WizardWeapon::Staff)
  //   .armor(Armor::Chainmail);
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; wizards are restricted to the wizard weapon subset
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .wizard()
  //   .weapon(Weapon::LongSword)
  //   .spell(Spell::Invisibility);
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; warriors cannot cast a spell
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .warrior()
  //   .weapon(Weapon::LongSword)
  //   .spell(Spell::Invisibility);
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; wizards cannot wear armor
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::builder()
  //   .wizard()
  //   .weapon(WizardWeapon::Staff)
  //   .armor(Armor::Chainmail);
  // ---------------------------------------------------------------------------
}