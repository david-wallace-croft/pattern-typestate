use self::typestate::Typestate;
use super::event::Event;
use std::mem;

// The StateMachine submodules are private
mod data;
mod ejected_state;
mod running_state;
mod state_trait;
mod stopped_state;
mod typestate;

#[cfg(test)]
mod test;

#[derive(Default)]
pub struct StateMachine {
  typestate: Typestate,
}

impl StateMachine {
  pub fn get_position(&self) -> usize {
    self
      .typestate
      .get_position()
  }

  pub fn get_state_name(&self) -> &'static str {
    self
      .typestate
      .get_state_name()
  }

  pub fn transit(
    &mut self,
    event: &Event,
  ) {
    // Doing this directly will not compile; cannot move and cannot copy because
    // Typestate uses Data which does not implement Copy.
    //
    // self.typestate = self.typestate.transit(event);

    let typestate: Typestate = mem::take(&mut self.typestate);

    self.typestate = typestate.transit(event);
  }
}
