use self::typestate::Typestate;
use super::event::Event;

// The StateMachine submodules are private
mod ejected;
mod player_data;
mod running;
mod state_trait;
mod stopped;
mod typestate;

#[cfg(test)]
mod test;

pub struct StateMachine {
  typestate_option: Option<Typestate>,
}

impl StateMachine {
  pub fn get_position(&self) -> usize {
    match &self.typestate_option {
      None => unreachable!(),
      Some(typestate) => typestate.get_position(),
    }
  }

  pub fn get_state_name(&self) -> &'static str {
    match &self.typestate_option {
      None => unreachable!(),
      Some(typestate) => typestate.get_state_name(),
    }
  }

  pub fn transit(
    &mut self,
    event: &Event,
  ) {
    // Will not compile; cannot move
    // self.typestate = self.typestate.transit(event);

    let typestate_option = self
      .typestate_option
      .take();

    let typestate_old: Typestate = typestate_option.unwrap_or_default();

    let typestate_new = typestate_old.transit(event);

    self
      .typestate_option
      .replace(typestate_new);
  }
}

impl Default for StateMachine {
  fn default() -> Self {
    Self {
      typestate_option: Some(Typestate::default()),
    }
  }
}
