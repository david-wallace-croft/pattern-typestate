#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub enum Armor {
  Chainmail,
  None,
  Plate,
}

#[derive(Debug, PartialEq)]
pub enum CharacterClass {
  None,
  Warrior,
  Wizard,
}

#[derive(Debug, PartialEq)]
pub enum Spell {
  Invisibility,
  None,
  Sleep,
}

#[derive(Debug, PartialEq)]
pub enum WarriorWeapon {
  Dagger,
  Mace,
  None,
  LongSword,
  Staff,
}

#[derive(Debug, PartialEq)]
pub enum WizardWeapon {
  Dagger,
  None,
  Staff,
}

#[derive(Debug, PartialEq)]
pub struct PlayerCharacter {
  armor: Armor,
  character_class: CharacterClass,
  spell: Spell,
  warrior_weapon: WarriorWeapon,
  wizard_weapon: WizardWeapon,
}

impl PlayerCharacter {
  fn private_default() -> Self {
    Self {
      armor: Armor::None,
      character_class: CharacterClass::None,
      spell: Spell::None,
      warrior_weapon: WarriorWeapon::None,
      wizard_weapon: WizardWeapon::None,
    }
  }

  // TODO: clippy
  // TODO: Show how initial weapon choice can constrain later choices
  // pub fn builder() -> StrictBuilderPlayerCharacter {
  //   StrictBuilderPlayerCharacter {
  //     player_character: PlayerCharacter::private_default(),
  //   }
  // }

  // TODO: make a builder() which returns StrictBuilderPlayerCharacter

  pub fn warrior_builder() -> StrictBuilderArmor {
    let mut player_character = PlayerCharacter::private_default();

    player_character.character_class = CharacterClass::Warrior;

    StrictBuilderArmor {
      player_character,
    }
  }

  pub fn wizard_builder() -> StrictBuilderSpell {
    let mut player_character = PlayerCharacter::private_default();

    player_character.character_class = CharacterClass::Wizard;

    StrictBuilderSpell {
      player_character,
    }
  }
}

pub struct StrictBuilderArmor {
  player_character: PlayerCharacter,
}

impl StrictBuilderArmor {
  pub fn armor(
    mut self,
    armor: Armor,
  ) -> StrictBuilderWarriorWeapon {
    self
      .player_character
      .armor = armor;

    StrictBuilderWarriorWeapon {
      player_character: self.player_character,
    }
  }
}

pub struct StrictBuilderWarriorWeapon {
  player_character: PlayerCharacter,
}

impl StrictBuilderWarriorWeapon {
  pub fn warrior_weapon(
    mut self,
    warrior_weapon: WarriorWeapon,
  ) -> PlayerCharacter {
    self
      .player_character
      .warrior_weapon = warrior_weapon;

    self.player_character
  }
}

pub struct StrictBuilderSpell {
  player_character: PlayerCharacter,
}

impl StrictBuilderSpell {
  pub fn spell(
    mut self,
    spell: Spell,
  ) -> StrictBuilderWizardWeapon {
    self
      .player_character
      .spell = spell;

    StrictBuilderWizardWeapon {
      player_character: self.player_character,
    }
  }
}

pub struct StrictBuilderWizardWeapon {
  player_character: PlayerCharacter,
}

impl StrictBuilderWizardWeapon {
  pub fn wizard_weapon(
    mut self,
    wizard_weapon: WizardWeapon,
  ) -> PlayerCharacter {
    self
      .player_character
      .wizard_weapon = wizard_weapon;

    self.player_character
  }
}
