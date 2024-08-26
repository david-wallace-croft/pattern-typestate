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
//! - Updated: 2024-08-26
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
use std::marker::PhantomData;

const DEFAULT_HEALTH_WARRIOR: isize = 10;
const DEFAULT_HEALTH_WIZARD: isize = 4;
const DEFAULT_WEALTH_WARRIOR: f64 = 5.;
const DEFAULT_WEALTH_WIZARD: f64 = 12.;
const DEFAULT_WISDOM_WARRIOR: usize = 11;
const DEFAULT_WISDOM_WIZARD: usize = 15;

pub struct StateCharacterClass;
pub struct StateHealth;
pub struct StateNote;
pub struct StateWarriorArmor;
pub struct StateWarriorWeapon;
pub struct StateWealth;
pub struct StateWisdom;
pub struct StateWizardSpell;
pub struct StateWizardWeapon;

//==============================================================================

pub struct PlayerCharacterConstructor<S> {
  player_character: PlayerCharacter,
  state: PhantomData<S>,
}

impl ConstructorCreator<PlayerCharacterConstructor<StateCharacterClass>>
  for PlayerCharacter
{
  // The public ConstructorCreator trait provides indirect access to the fields
  fn constructor() -> PlayerCharacterConstructor<StateCharacterClass> {
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
      state: PhantomData,
    }
  }
}

//==============================================================================

impl PlayerCharacterConstructor<StateCharacterClass> {
  pub fn warrior(mut self) -> PlayerCharacterConstructor<StateWarriorWeapon> {
    self
      .player_character
      .character_class = CharacterClass::Warrior;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }

  pub fn wizard(mut self) -> PlayerCharacterConstructor<StateWizardWeapon> {
    self
      .player_character
      .character_class = CharacterClass::Wizard;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }
}

//==============================================================================

impl PlayerCharacterConstructor<StateWarriorWeapon> {
  pub fn weapon(
    mut self,
    weapon: Weapon,
  ) -> PlayerCharacterConstructor<StateWarriorArmor> {
    self
      .player_character
      .weapon = weapon;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }
}

//==============================================================================

impl PlayerCharacterConstructor<StateWarriorArmor> {
  pub fn armor(
    mut self,
    armor: Armor,
  ) -> PlayerCharacterConstructor<StateHealth> {
    self
      .player_character
      .armor = armor;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }
}

//==============================================================================

impl PlayerCharacterConstructor<StateWizardWeapon> {
  pub fn weapon(
    mut self,
    wizard_weapon: WizardWeapon,
  ) -> PlayerCharacterConstructor<StateWizardSpell> {
    let weapon: Weapon = wizard_weapon.into();

    self
      .player_character
      .weapon = weapon;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }
}

//==============================================================================

impl PlayerCharacterConstructor<StateWizardSpell> {
  pub fn spell(
    mut self,
    spell: Spell,
  ) -> PlayerCharacterConstructor<StateHealth> {
    self
      .player_character
      .spell = spell;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }
}

//==============================================================================

impl PlayerCharacterConstructor<StateHealth> {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> PlayerCharacter {
    self
      .health_default()
      .construct()
  }

  pub fn health(
    mut self,
    health: isize,
  ) -> PlayerCharacterConstructor<StateWealth> {
    self
      .player_character
      .health = health;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }

  /// Use the character class-specific default value for health
  pub fn health_default(self) -> PlayerCharacterConstructor<StateWealth> {
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

impl PlayerCharacterConstructor<StateWealth> {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> PlayerCharacter {
    self
      .wealth_default()
      .construct()
  }

  pub fn wealth(
    mut self,
    wealth: f64,
  ) -> PlayerCharacterConstructor<StateWisdom> {
    self
      .player_character
      .wealth = wealth;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }

  /// Use the character class-specific default value for wealth
  pub fn wealth_default(self) -> PlayerCharacterConstructor<StateWisdom> {
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

impl PlayerCharacterConstructor<StateWisdom> {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> PlayerCharacter {
    self
      .wisdom_default()
      .construct()
  }

  pub fn wisdom(
    mut self,
    wisdom: usize,
  ) -> PlayerCharacterConstructor<StateNote> {
    self
      .player_character
      .wisdom = wisdom;

    PlayerCharacterConstructor {
      player_character: self.player_character,
      state: PhantomData,
    }
  }

  /// Use the character class-specific default value for wisdom
  pub fn wisdom_default(self) -> PlayerCharacterConstructor<StateNote> {
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

impl PlayerCharacterConstructor<StateNote> {
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
