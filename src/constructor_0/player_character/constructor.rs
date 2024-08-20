//==============================================================================
//! An example of a fluent constructor that uses the typestate pattern.
//!
//! Always constructs a valid configuration of a PlayerCharacter:
//! - The character class may be either Warrior or Wizard
//! - Warriors can use any weapon
//! - Wizards can only use a subset of the weapons
//! - Warriors can wear armor but Wizards cannot
//! - Wizards can cast a spell but Warriors cannot
//! - Character class-specific default options for health, wealth, and wisdom
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-08-17
//! - Updated: 2024-08-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::super::armor::Armor;
use super::super::character_class::CharacterClass;
use super::super::constructor_creator::ConstructorCreator;
use super::super::player_character::PlayerCharacter;
use super::super::spell::Spell;
use super::super::weapon::Weapon;
use super::super::wizard_weapon::WizardWeapon;

const DEFAULT_HEALTH_WARRIOR: isize = 10;
const DEFAULT_HEALTH_WIZARD: isize = 4;
const DEFAULT_WEALTH_WARRIOR: f64 = 5.;
const DEFAULT_WEALTH_WIZARD: f64 = 12.;
const DEFAULT_WISDOM_WARRIOR: usize = 11;
const DEFAULT_WISDOM_WIZARD: usize = 15;

//==============================================================================

impl ConstructorCreator<PlayerCharacterConstructor> for PlayerCharacter {
  // The public ConstructorCreator trait provides indirect access to the fields
  fn constructor() -> PlayerCharacterConstructor {
    // The constructor submodule has direct access to the private fields
    let player_character = PlayerCharacter {
      armor: Armor::None,
      character_class: CharacterClass::None,
      health: 0,
      spell: Spell::None,
      wealth: 0.,
      weapon: Weapon::None,
      wisdom: 0,
    };

    PlayerCharacterConstructor::new(player_character)
  }
}

//==============================================================================

pub struct PlayerCharacterConstructor {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructor {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn warrior(mut self) -> PlayerCharacterConstructorWarriorWeapon {
    self
      .player_character
      .character_class = CharacterClass::Warrior;

    PlayerCharacterConstructorWarriorWeapon::new(self.player_character)
  }

  pub fn wizard(mut self) -> PlayerCharacterConstructorWizardWeapon {
    self
      .player_character
      .character_class = CharacterClass::Wizard;

    PlayerCharacterConstructorWizardWeapon::new(self.player_character)
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWarriorWeapon {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWarriorWeapon {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn weapon(
    mut self,
    weapon: Weapon,
  ) -> PlayerCharacterConstructorWarriorArmor {
    self
      .player_character
      .weapon = weapon;

    PlayerCharacterConstructorWarriorArmor::new(self.player_character)
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWarriorArmor {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWarriorArmor {
  pub fn armor(
    mut self,
    armor: Armor,
  ) -> PlayerCharacterConstructorHealth {
    self
      .player_character
      .armor = armor;

    PlayerCharacterConstructorHealth::new(self.player_character)
  }

  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWizardWeapon {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWizardWeapon {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn weapon(
    mut self,
    wizard_weapon: WizardWeapon,
  ) -> PlayerCharacterConstructorWizardSpell {
    let weapon: Weapon = wizard_weapon.into();

    self
      .player_character
      .weapon = weapon;

    PlayerCharacterConstructorWizardSpell::new(self.player_character)
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWizardSpell {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWizardSpell {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn spell(
    mut self,
    spell: Spell,
  ) -> PlayerCharacterConstructorHealth {
    self
      .player_character
      .spell = spell;

    PlayerCharacterConstructorHealth::new(self.player_character)
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorHealth {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorHealth {
  /// Use the character class-specific default values for the remaining fields
  pub fn default(self) -> PlayerCharacter {
    self
      .health_default()
      .default()
  }

  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn health(
    mut self,
    health: isize,
  ) -> PlayerCharacterConstructorWealth {
    self
      .player_character
      .health = health;

    PlayerCharacterConstructorWealth::new(self.player_character)
  }

  /// Use the character class-specific default value for health
  pub fn health_default(self) -> PlayerCharacterConstructorWealth {
    match self
      .player_character
      .character_class
    {
      CharacterClass::None => unreachable!(),
      CharacterClass::Warrior => self.health(DEFAULT_HEALTH_WARRIOR),
      CharacterClass::Wizard => self.health(DEFAULT_HEALTH_WIZARD),
    }
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWealth {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWealth {
  /// Use the character class-specific default values for the remaining fields
  pub fn default(self) -> PlayerCharacter {
    self
      .wealth_default()
      .default()
  }

  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn wealth(
    mut self,
    wealth: f64,
  ) -> PlayerCharacterConstructorWisdom {
    self
      .player_character
      .wealth = wealth;

    PlayerCharacterConstructorWisdom::new(self.player_character)
  }

  /// Use the character class-specific default value for wealth
  pub fn wealth_default(self) -> PlayerCharacterConstructorWisdom {
    match self
      .player_character
      .character_class
    {
      CharacterClass::None => unreachable!(),
      CharacterClass::Warrior => self.wealth(DEFAULT_WEALTH_WARRIOR),
      CharacterClass::Wizard => self.wealth(DEFAULT_WEALTH_WIZARD),
    }
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWisdom {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWisdom {
  /// Use the character class-specific default values for the remaining fields
  pub fn default(self) -> PlayerCharacter {
    self.wisdom_default()
  }

  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn wisdom(
    mut self,
    wisdom: usize,
  ) -> PlayerCharacter {
    self
      .player_character
      .wisdom = wisdom;

    self.player_character
  }

  /// Use the character class-specific default value for wisdom
  pub fn wisdom_default(self) -> PlayerCharacter {
    match self
      .player_character
      .character_class
    {
      CharacterClass::None => unreachable!(),
      CharacterClass::Warrior => self.wisdom(DEFAULT_WISDOM_WARRIOR),
      CharacterClass::Wizard => self.wisdom(DEFAULT_WISDOM_WIZARD),
    }
  }
}
