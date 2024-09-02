use self::typestate::Typestate;
use super::data::Data;
use super::event::Event;

// The StateMachine submodules are private
mod ejected;
mod running;
mod state_operator;
mod state_trait;
mod stopped;
mod typestate;

#[cfg(test)]
mod test;

pub struct StateMachine {
  typestate_option: Option<Typestate>,
}

impl StateMachine {
  pub fn get_state_name(&self) -> &'static str {
    match &self.typestate_option {
      None => unreachable!(),
      Some(typestate) => typestate.get_state_name(),
    }
  }

  pub fn transit(
    &mut self,
    data: &mut Data,
    event: Event,
  ) {
    // Will not compile; cannot move
    // self.typestate = self.typestate.transit(request);

    let typestate_option = self
      .typestate_option
      .take();

    let typestate_old: Typestate = typestate_option.unwrap_or_default();

    let typestate_new = match typestate_old {
      Typestate::Ejected(state_operator) => state_operator.transit(event),
      Typestate::Running(state_operator) => state_operator.transit(data, event),
      Typestate::Stopped(state_operator) => state_operator.transit(data, event),
    };

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
