use self::typestate::Typestate;
use super::data::Data;
use super::event::Event;

// The StateMachine submodules are private
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
  pub fn get_state_name(&self) -> &'static str {
    self
      .typestate
      .get_state_name()
  }

  pub fn transit(
    &mut self,
    data: &mut Data,
    event: &Event,
  ) {
    self.typestate = self
      .typestate
      .transit(data, event);
  }
}
