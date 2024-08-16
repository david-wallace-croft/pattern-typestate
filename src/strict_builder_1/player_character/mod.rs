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
pub enum WizardWeapon {
  Dagger,
  None,
  Staff,
}

#[derive(Debug, PartialEq)]
pub enum Weapon {
  Dagger,
  Mace,
  None,
  LongSword,
  Staff,
}

#[derive(Debug, PartialEq)]
pub struct PlayerCharacter {
  armor: Armor,
  character_class: CharacterClass,
  spell: Spell,
  weapon: Weapon,
}

impl PlayerCharacter {
  pub fn builder() -> StrictBuilderPlayerCharacter {
    StrictBuilderPlayerCharacter {
      player_character: PlayerCharacter {
        armor: Armor::None,
        character_class: CharacterClass::None,
        spell: Spell::None,
        weapon: Weapon::None,
      },
    }
  }
}

pub struct StrictBuilderPlayerCharacter {
  player_character: PlayerCharacter,
}

impl StrictBuilderPlayerCharacter {
  pub fn warrior(mut self) -> StrictBuilderWarriorWeapon {
    self
      .player_character
      .character_class = CharacterClass::Warrior;

    StrictBuilderWarriorWeapon {
      player_character: self.player_character,
    }
  }

  pub fn wizard(mut self) -> StrictBuilderWizardWeapon {
    self
      .player_character
      .character_class = CharacterClass::Wizard;

    StrictBuilderWizardWeapon {
      player_character: self.player_character,
    }
  }
}

pub struct StrictBuilderWarriorWeapon {
  player_character: PlayerCharacter,
}

impl StrictBuilderWarriorWeapon {
  pub fn weapon(
    mut self,
    weapon: Weapon,
  ) -> StrictBuilderArmor {
    self
      .player_character
      .weapon = weapon;

    StrictBuilderArmor {
      player_character: self.player_character,
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
  ) -> PlayerCharacter {
    self
      .player_character
      .armor = armor;

    self.player_character
  }
}

pub struct StrictBuilderWizardWeapon {
  player_character: PlayerCharacter,
}

impl StrictBuilderWizardWeapon {
  pub fn weapon(
    mut self,
    wizard_weapon: WizardWeapon,
  ) -> StrictBuilderSpell {
    let weapon: Weapon = match wizard_weapon {
      WizardWeapon::Dagger => Weapon::Dagger,
      WizardWeapon::None => Weapon::None,
      WizardWeapon::Staff => Weapon::Staff,
    };

    self
      .player_character
      .weapon = weapon;

    StrictBuilderSpell {
      player_character: self.player_character,
    }
  }
}

pub struct StrictBuilderSpell {
  player_character: PlayerCharacter,
}

impl StrictBuilderSpell {
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
