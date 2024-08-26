use super::armor::Armor;
use super::character_class::CharacterClass;
use super::constructor_creator::ConstructorCreator;
use super::player_character::constructor::{
  PlayerCharacterConstructor, StateNote,
};
use super::player_character::PlayerCharacter;
use super::spell::Spell;
use super::weapon::Weapon;
use super::wizard_weapon::WizardWeapon;

const TEST_NOTES: [&str; 3] = [
  "Afraid of heights",
  "Enjoys puns",
  "Raised in a fishing village",
];

#[test]
fn test_constructor_0() {
  let actual = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health(10)
    .wealth(11.)
    .wisdom(12)
    .note("Afraid of heights")
    .construct();

  assert_eq!(actual.armor(), Armor::Chainmail);

  assert_eq!(actual.character_class(), CharacterClass::Warrior);

  assert_eq!(actual.health(), 10);

  assert_eq!(actual.notes(), vec!["Afraid of heights".to_string()]);

  assert_eq!(actual.spell(), Spell::None);

  assert_eq!(actual.weapon(), Weapon::LongSword);

  assert_eq!(actual.wealth(), 11.);

  assert_eq!(actual.wisdom(), 12);
}

#[test]
fn test_constructor_1() {
  let actual = PlayerCharacter::constructor()
    .wizard()
    .weapon(WizardWeapon::Dagger)
    .spell(Spell::Invisibility)
    .construct();

  assert_eq!(actual.armor(), Armor::None);

  assert_eq!(actual.character_class(), CharacterClass::Wizard);

  assert_eq!(actual.health(), 4);

  assert_eq!(actual.notes(), Vec::<String>::new());

  assert_eq!(actual.spell(), Spell::Invisibility);

  assert_eq!(actual.weapon(), Weapon::Dagger);

  assert_eq!(actual.wealth(), 12.);

  assert_eq!(actual.wisdom(), 15);
}

#[test]
fn test_constructor_2() {
  let expected_notes: Vec<String> = TEST_NOTES
    .iter()
    .map(|s| s.to_string())
    .collect();

  let actual_player_character: PlayerCharacter = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health(11)
    .wealth(12.)
    .wisdom(13)
    .note(TEST_NOTES[0])
    .note(TEST_NOTES[1])
    .note(TEST_NOTES[2])
    .construct();

  let actual_notes: Vec<String> = actual_player_character.notes();

  assert_eq!(actual_notes, expected_notes);
}

#[test]
fn test_constructor_3() {
  let expected_notes: Vec<String> = TEST_NOTES
    .iter()
    .map(|s| s.to_string())
    .collect();

  let mut player_character_constructor_note: PlayerCharacterConstructor<
    StateNote,
  > = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health(11)
    .wealth(12.)
    .wisdom(13);

  for note in TEST_NOTES {
    player_character_constructor_note =
      player_character_constructor_note.note(note);
  }

  let actual_player_character: PlayerCharacter =
    player_character_constructor_note.construct();

  let actual_notes: Vec<String> = actual_player_character.notes();

  assert_eq!(actual_notes, expected_notes);
}
