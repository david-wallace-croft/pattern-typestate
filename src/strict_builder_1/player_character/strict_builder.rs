use super::super::armor::Armor;
use super::super::character_class::CharacterClass;
use super::super::player_character::PlayerCharacter;
use super::super::spell::Spell;
use super::super::weapon::Weapon;
use super::super::wizard_weapon::WizardWeapon;

pub struct StrictBuilderPlayerCharacter {
  player_character: PlayerCharacter,
}

impl StrictBuilderPlayerCharacter {
  // The non-fluent constructor is only accessible to the super-module
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
  // The non-fluent constructor is only accessible to this module
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
  ) -> PlayerCharacter {
    self
      .player_character
      .armor = armor;

    self.player_character
  }

  // The non-fluent constructor is only accessible to this module
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
  // The non-fluent constructor is only accessible to this module
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
  // The non-fluent constructor is only accessible to this module
  fn new(player_character: PlayerCharacter) -> Self {
    Self {
      player_character,
    }
  }

  pub fn spell(
    mut self,
    spell: Spell,
  ) -> PlayerCharacter {
    self
      .player_character
      .spell = spell;

    self.player_character
  }
}
