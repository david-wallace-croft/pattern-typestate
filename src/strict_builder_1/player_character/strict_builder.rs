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

pub struct StrictBuilderPlayerCharacter {
  player_character: PlayerCharacter,
}

impl StrictBuilderPlayerCharacter {
  // The static constructor is only accessible to the super-module
  pub(super) fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn warrior(mut self) -> StrictBuilderWarriorWeapon {
    self
      .player_character
      .character_class = CharacterClass::Warrior;

    StrictBuilderWarriorWeapon::new(self.player_character)
  }

  pub fn wizard(mut self) -> StrictBuilderWizardWeapon {
    self
      .player_character
      .character_class = CharacterClass::Wizard;

    StrictBuilderWizardWeapon::new(self.player_character)
  }
}

pub struct StrictBuilderWarriorWeapon {
  player_character: PlayerCharacter,
}

impl StrictBuilderWarriorWeapon {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn weapon(
    mut self,
    weapon: Weapon,
  ) -> StrictBuilderArmor {
    self
      .player_character
      .weapon = weapon;

    StrictBuilderArmor::new(self.player_character)
  }
}

pub struct StrictBuilderArmor {
  player_character: PlayerCharacter,
}

impl StrictBuilderArmor {
  pub fn armor(
    mut self,
    armor: Armor,
  ) -> StrictBuilderHealth {
    self
      .player_character
      .armor = armor;

    StrictBuilderHealth::new(self.player_character)
  }

  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }
}

pub struct StrictBuilderWizardWeapon {
  player_character: PlayerCharacter,
}

impl StrictBuilderWizardWeapon {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn weapon(
    mut self,
    wizard_weapon: WizardWeapon,
  ) -> StrictBuilderSpell {
    let weapon: Weapon = wizard_weapon.into();

    self
      .player_character
      .weapon = weapon;

    StrictBuilderSpell::new(self.player_character)
  }
}

pub struct StrictBuilderSpell {
  player_character: PlayerCharacter,
}

impl StrictBuilderSpell {
  // The static constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn spell(
    mut self,
    spell: Spell,
  ) -> StrictBuilderHealth {
    self
      .player_character
      .spell = spell;

    StrictBuilderHealth::new(self.player_character)
  }
}

pub struct StrictBuilderHealth {
  player_character: PlayerCharacter,
}

impl StrictBuilderHealth {
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
  ) -> StrictBuilderWealth {
    self
      .player_character
      .health = health;

    StrictBuilderWealth::new(self.player_character)
  }

  /// Use the character class-specific default value for health
  pub fn health_default(self) -> StrictBuilderWealth {
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

pub struct StrictBuilderWealth {
  player_character: PlayerCharacter,
}

impl StrictBuilderWealth {
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
  ) -> StrictBuilderWisdom {
    self
      .player_character
      .wealth = wealth;

    StrictBuilderWisdom::new(self.player_character)
  }

  /// Use the character class-specific default value for wealth
  pub fn wealth_default(self) -> StrictBuilderWisdom {
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

pub struct StrictBuilderWisdom {
  player_character: PlayerCharacter,
}

impl StrictBuilderWisdom {
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
