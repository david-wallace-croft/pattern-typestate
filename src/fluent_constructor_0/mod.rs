use armor::Armor;
use player_character::PlayerCharacter;
use spell::Spell;
use weapon::Weapon;
use wizard_weapon::WizardWeapon;

pub mod armor;
pub mod character_class;
pub mod player_character;
pub mod spell;
pub mod weapon;
pub mod wizard_weapon;

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

  // Use the character-specific default value for health
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health_default()
    .wealth(10.)
    .wisdom(10);

  // Use the character-specific default values for health, wealth, and wisdom
  let _player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .default();

  // TODO: update examples for new fields

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
