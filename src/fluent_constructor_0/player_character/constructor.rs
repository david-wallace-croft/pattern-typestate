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
//! - Can have zero or more notes describing the character
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-08-17
//! - Updated: 2024-08-24
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
      armor: Default::default(),
      character_class: Default::default(),
      health: Default::default(),
      notes: Default::default(),
      spell: Default::default(),
      wealth: Default::default(),
      weapon: Default::default(),
      wisdom: Default::default(),
    };

    PlayerCharacterConstructor {
      player_character,
    }
  }
}

//==============================================================================

pub struct PlayerCharacterConstructor {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructor {
  pub fn warrior(mut self) -> PlayerCharacterConstructorWarriorWeapon {
    self
      .player_character
      .character_class = CharacterClass::Warrior;

    PlayerCharacterConstructorWarriorWeapon {
      player_character: self.player_character,
    }
  }

  pub fn wizard(mut self) -> PlayerCharacterConstructorWizardWeapon {
    self
      .player_character
      .character_class = CharacterClass::Wizard;

    PlayerCharacterConstructorWizardWeapon {
      player_character: self.player_character,
    }
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWarriorWeapon {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWarriorWeapon {
  pub fn weapon(
    mut self,
    weapon: Weapon,
  ) -> PlayerCharacterConstructorWarriorArmor {
    self
      .player_character
      .weapon = weapon;

    PlayerCharacterConstructorWarriorArmor {
      player_character: self.player_character,
    }
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

    PlayerCharacterConstructorHealth {
      player_character: self.player_character,
    }
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWizardWeapon {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWizardWeapon {
  pub fn weapon(
    mut self,
    wizard_weapon: WizardWeapon,
  ) -> PlayerCharacterConstructorWizardSpell {
    let weapon: Weapon = wizard_weapon.into();

    self
      .player_character
      .weapon = weapon;

    PlayerCharacterConstructorWizardSpell {
      player_character: self.player_character,
    }
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorWizardSpell {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorWizardSpell {
  pub fn spell(
    mut self,
    spell: Spell,
  ) -> PlayerCharacterConstructorHealth {
    self
      .player_character
      .spell = spell;

    PlayerCharacterConstructorHealth {
      player_character: self.player_character,
    }
  }
}

//==============================================================================

pub struct PlayerCharacterConstructorHealth {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorHealth {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> PlayerCharacter {
    self
      .health_default()
      .construct()
  }

  pub fn health(
    mut self,
    health: isize,
  ) -> PlayerCharacterConstructorWealth {
    self
      .player_character
      .health = health;

    PlayerCharacterConstructorWealth {
      player_character: self.player_character,
    }
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
  /// Use the default values for the remaining fields
  pub fn construct(self) -> PlayerCharacter {
    self
      .wealth_default()
      .construct()
  }

  pub fn wealth(
    mut self,
    wealth: f64,
  ) -> PlayerCharacterConstructorWisdom {
    self
      .player_character
      .wealth = wealth;

    PlayerCharacterConstructorWisdom {
      player_character: self.player_character,
    }
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
  /// Use the default values for the remaining fields
  pub fn construct(self) -> PlayerCharacter {
    self
      .wisdom_default()
      .construct()
  }

  pub fn wisdom(
    mut self,
    wisdom: usize,
  ) -> PlayerCharacterConstructorNote {
    self
      .player_character
      .wisdom = wisdom;

    PlayerCharacterConstructorNote {
      player_character: self.player_character,
    }
  }

  /// Use the character class-specific default value for wisdom
  pub fn wisdom_default(self) -> PlayerCharacterConstructorNote {
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

//==============================================================================

pub struct PlayerCharacterConstructorNote {
  player_character: PlayerCharacter,
}

impl PlayerCharacterConstructorNote {
  /// Completes the construction of the PlayerCharacter
  pub fn construct(self) -> PlayerCharacter {
    self.player_character
  }

  /// Adds a note to the PlayerCharacter notes
  pub fn note(
    mut self,
    note: &str,
  ) -> Self {
    self
      .player_character
      .notes
      .push(note.into());

    self
  }
}
