use super::armor::Armor;
use super::character_class::CharacterClass;
use super::constructor_creator::ConstructorCreator;
use super::player_character::PlayerCharacter;
use super::spell::Spell;
use super::weapon::Weapon;
use super::wizard_weapon::WizardWeapon;

#[test]
fn test_constructor_0() {
  let actual = PlayerCharacter::constructor()
    .warrior()
    .weapon(Weapon::LongSword)
    .armor(Armor::Chainmail)
    .health(10)
    .wealth(11.)
    .wisdom(12);

  assert_eq!(actual.armor(), Armor::Chainmail);

  assert_eq!(actual.character_class(), CharacterClass::Warrior);

  assert_eq!(actual.health(), 10);

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
    .default();

  assert_eq!(actual.armor(), Armor::None);

  assert_eq!(actual.character_class(), CharacterClass::Wizard);

  assert_eq!(actual.health(), 4);

  assert_eq!(actual.spell(), Spell::Invisibility);

  assert_eq!(actual.weapon(), Weapon::Dagger);

  assert_eq!(actual.wealth(), 12.);

  assert_eq!(actual.wisdom(), 15);
}
