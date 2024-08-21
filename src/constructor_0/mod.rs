//==============================================================================
//! An example of using a fluent constructor that uses the typestate pattern.
//!
//! The commented-out code shows what cannot be compiled:
//! - Using a struct literal
//! - Using the Default trait
//! - Using a static constructor
//! - Using the default() mutator method before all required fields are set
//! - Setting the fields out of prerequisite order
//! - Using the wrong set of values for a field based on a previously set field
//! - Setting a field that is not allowed based on a previously set field
//! - Reusing a fluent constructor after it has been used
//! - Reusing a fluent constructor method chain fragment after it has been used
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-08-14
//! - Updated: 2024-08-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

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
  // A warrior can use any weapon and can wear armor
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health(10)
    .wealth(10.)
    .wisdom(10);

  // A wizard can use a wizard weapon and can cast spells
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

  // Provides values only where required and uses default values for the rest
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .default();

  // TODO: Example where you do have to call construct() for the last one
  //   because a method can be called more than once.

  // ===========================================================================
  // Does not compile; cannot use a structure literal because fields are private
  //
  // let _player_character: PlayerCharacter = PlayerCharacter {
  //   armor: Armor::Chainmail,
  //   character_class: CharacterClass::Wizard,
  //   health: 10,
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
  //   .default();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; the character class must be chosen first
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .weapon(Weapon::LongSword)
  //   .wizard()
  //   .spell(Spell::Invisibility)
  //   .default();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; warriors are not restricted to the wizard weapon subset
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .warrior()
  //   .weapon(WizardWeapon::Staff)
  //   .armor(Armor::Chainmail)
  //   .default();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; wizards are restricted to the wizard weapon subset
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .wizard()
  //   .weapon(Weapon::LongSword)
  //   .spell(Spell::Invisibility)
  //   .default();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; warriors cannot cast a spell
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .warrior()
  //   .weapon(Weapon::LongSword)
  //   .spell(Spell::Invisibility)
  //   .default();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; wizards cannot wear armor
  //
  // let _player_character: PlayerCharacter = PlayerCharacter::constructor()
  //   .wizard()
  //   .weapon(WizardWeapon::Staff)
  //   .armor(Armor::Chainmail)
  //   .default();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; cannot reuse a fluent constructor
  //
  // let player_character_constructor = PlayerCharacter::constructor();
  //
  // let _player_character: PlayerCharacter = player_character_constructor
  //   .warrior()
  //   .weapon(Weapon::LongSword)
  //   .armor(Armor::Chainmail)
  //   .default();
  //
  // let _player_character: PlayerCharacter = player_character_constructor
  //   .wizard()
  //   .weapon(WizardWeapon::Staff)
  //   .spell(Spell::Invisibility)
  //   .default();
  // ---------------------------------------------------------------------------

  // ===========================================================================
  // Does not compile; cannot reuse a fluent constructor method chain fragment
  //
  // let player_character_constructor_fragment = PlayerCharacter::constructor()
  //   .warrior()
  //   .weapon(Weapon::LongSword)
  //   .armor(Armor::Chainmail);
  //
  // let _player_character: PlayerCharacter
  //   = player_character_constructor_fragment.default();
  //
  // let _player_character: PlayerCharacter
  //   = player_character_constructor_fragment.default();
  // ---------------------------------------------------------------------------
}
