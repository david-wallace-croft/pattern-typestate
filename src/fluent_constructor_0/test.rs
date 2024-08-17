use super::armor::Armor;
use super::character_class::CharacterClass;
use super::player_character::PlayerCharacter;
use super::spell::Spell;
use super::weapon::Weapon;

#[test]
fn test0() {
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
