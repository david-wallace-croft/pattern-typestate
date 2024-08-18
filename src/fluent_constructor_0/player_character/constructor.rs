//==============================================================================
//! A fluent constructor for PlayerCharacter.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-08-18
//! - Updated: 2024-08-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::super::armor::Armor;
use super::super::character_class::CharacterClass;
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

// TODO: Separate structs with dashed line comments

pub struct ConstructorPlayerCharacter {
  player_character: PlayerCharacter,
}

impl ConstructorPlayerCharacter {
  // The static constructor is only accessible to the super-module
  pub(super) fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn warrior(mut self) -> ConstructorWarriorWeapon {
    self
      .player_character
      .character_class = CharacterClass::Warrior;

    ConstructorWarriorWeapon::new(self.player_character)
  }

  pub fn wizard(mut self) -> ConstructorWizardWeapon {
    self
      .player_character
      .character_class = CharacterClass::Wizard;

    ConstructorWizardWeapon::new(self.player_character)
  }
}

pub struct ConstructorWarriorWeapon {
  player_character: PlayerCharacter,
}

impl ConstructorWarriorWeapon {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn weapon(
    mut self,
    weapon: Weapon,
  ) -> ConstructorArmor {
    self
      .player_character
      .weapon = weapon;

    ConstructorArmor::new(self.player_character)
  }
}

pub struct ConstructorArmor {
  player_character: PlayerCharacter,
}

impl ConstructorArmor {
  pub fn armor(
    mut self,
    armor: Armor,
  ) -> ConstructorHealth {
    self
      .player_character
      .armor = armor;

    ConstructorHealth::new(self.player_character)
  }

  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }
}

pub struct ConstructorWizardWeapon {
  player_character: PlayerCharacter,
}

impl ConstructorWizardWeapon {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn weapon(
    mut self,
    wizard_weapon: WizardWeapon,
  ) -> ConstructorSpell {
    let weapon: Weapon = wizard_weapon.into();

    self
      .player_character
      .weapon = weapon;

    ConstructorSpell::new(self.player_character)
  }
}

pub struct ConstructorSpell {
  player_character: PlayerCharacter,
}

impl ConstructorSpell {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn spell(
    mut self,
    spell: Spell,
  ) -> ConstructorHealth {
    self
      .player_character
      .spell = spell;

    ConstructorHealth::new(self.player_character)
  }
}

pub struct ConstructorHealth {
  player_character: PlayerCharacter,
}

impl ConstructorHealth {
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
  ) -> ConstructorWealth {
    self
      .player_character
      .health = health;

    ConstructorWealth::new(self.player_character)
  }

  /// Use the character class-specific default value for health
  pub fn health_default(self) -> ConstructorWealth {
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

pub struct ConstructorWealth {
  player_character: PlayerCharacter,
}

impl ConstructorWealth {
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
  ) -> ConstructorWisdom {
    self
      .player_character
      .wealth = wealth;

    ConstructorWisdom::new(self.player_character)
  }

  /// Use the character class-specific default value for wealth
  pub fn wealth_default(self) -> ConstructorWisdom {
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

pub struct ConstructorWisdom {
  player_character: PlayerCharacter,
}

impl ConstructorWisdom {
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
  pub fn wisdom_default(mut self) -> PlayerCharacter {
    self
      .player_character
      .wisdom = match self
      .player_character
      .character_class
    {
      CharacterClass::None => unreachable!(),
      CharacterClass::Warrior => DEFAULT_WISDOM_WARRIOR,
      CharacterClass::Wizard => DEFAULT_WISDOM_WIZARD,
    };

    self.player_character
  }
}