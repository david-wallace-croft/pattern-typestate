//==============================================================================
//! An example of using a fluent constructor that uses the typestate pattern.
//!
//! The commented-out code shows what cannot be compiled:
//! - Using a struct literal
//! - Using the Default trait
//! - Using a static constructor
//! - Using the construct() method before all required fields are set
//! - Setting the fields out of prerequisite order
//! - Using the wrong set of values for a field based on a previously set field
//! - Setting a field that is not allowed based on a previously set field
//! - Reusing a fluent constructor after construction completion
//! - Reusing a fluent constructor fragment after construction completion
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-08-14
//! - Updated: 2024-08-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use self::armor::Armor;
use self::constructor_creator::ConstructorCreator;
use self::player_character::constructor::PlayerCharacterConstructorNote;
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
  // A warrior can use any weapon and can wear armor
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health(10)
    .wealth(10.)
    .wisdom(10)
    .note("Afraid of heights")
    .construct();

  // A wizard can use a wizard weapon and can cast spells
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .wizard()
    .weapon(WizardWeapon::Staff)
    .spell(Spell::Invisibility)
    .health(4)
    .wealth(20.)
    .wisdom(15)
    .note("Enjoys puns")
    .construct();

  // Some fields such as health have character class-specific default values
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health_default()
    .wealth(10.)
    .wisdom(10)
    .note("Raised in a fishing village")
    .construct();

  // Provides values only where required and uses default values for the rest
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .construct();

  // Some fields can have multiple values
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health(10)
    .wealth(10.)
    .wisdom(10)
    .note("Afraid of heights")
    .note("Enjoys puns")
    .note("Raised in a fishing village")
    .construct();

  // A constructor fragment can be used when iterating over values for a field
  let mut player_character_constructor_note: PlayerCharacterConstructorNote =
    PlayerCharacter::constructor()
      .warrior()
      .weapon(Weapon::LongSword)
      .armor(Armor::Chainmail)
      .health(10)
      .wealth(10.)
      .wisdom(10);

  for note in [
    "Afraid of heights",
    "Enjoys puns",
    "Raised in a fishing village",
  ] {
    player_character_constructor_note =
      player_character_constructor_note.note(note);
  }

  let _player_character = player_character_constructor_note.construct();

  // ===========================================================================
  // Does not compile; cannot use a structure literal because fields are private
  //
  // let _player_character: PlayerCharacter = PlayerCharacter {
  //   armor: Armor::Chainmail,
  //   character_class: CharacterClass::Wizard,
  //   health: 10,
  //   notes: vec!["Afraid of heights".to_string()],
  //   spell: Spell::Invisibility,
  //   wealth: 11.,
  //   weapon: Weapon::LongSword,
  //   wisdom: 12,
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
  //   10,
  //   vec!["Afraid of heights".to_string()],
  //   Spell::Invisibility,
  //   11.,
  //   Weapon::LongSword,
  //   12,
  // );
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; some fields are required and do not have a default option
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .construct();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; the character class must be chosen before the weapon
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .weapon(Weapon::LongSword)
  //   .wizard()
  //   .spell(Spell::Invisibility)
  //   .construct();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; warriors are not restricted to the wizard weapon subset
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .warrior()
  //   .weapon(WizardWeapon::Staff)
  //   .armor(Armor::Chainmail)
  //   .construct();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; wizards are restricted to the wizard weapon subset
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .wizard()
  //   .weapon(Weapon::LongSword)
  //   .spell(Spell::Invisibility)
  //   .construct();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; warriors cannot cast a spell
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .warrior()
  //   .weapon(Weapon::LongSword)
  //   .spell(Spell::Invisibility)
  //   .construct();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; wizards cannot wear armor
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .wizard()
  //   .weapon(WizardWeapon::Staff)
  //   .armor(Armor::Chainmail)
  //   .construct();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; cannot reuse a constructor after construction
  //
  // let player_character_constructor = PlayerCharacter::constructor();
  //
  // let _player_character: PlayerCharacter = player_character_constructor
  //   .warrior()
  //   .weapon(Weapon::LongSword)
  //   .armor(Armor::Chainmail)
  //   .construct();
  //
  // let _player_character: PlayerCharacter = player_character_constructor
  //   .wizard()
  //   .weapon(WizardWeapon::Staff)
  //   .spell(Spell::Invisibility)
  //   .construct();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; cannot reuse a constructor fragment after construction
  //
  // let player_character_constructor_fragment = PlayerCharacter::constructor()
  //   .warrior()
  //   .weapon(Weapon::LongSword)
  //   .armor(Armor::Chainmail);
  //
  // let _player_character: PlayerCharacter
  //   = player_character_constructor_fragment.construct();
  //
  // let _player_character: PlayerCharacter
  //   = player_character_constructor_fragment.construct();
  // ---------------------------------------------------------------------------
}
