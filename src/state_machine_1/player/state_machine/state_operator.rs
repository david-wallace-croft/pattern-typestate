use super::super::data::Data;
use super::ejected_state::EjectedState;
use super::running_state::RunningState;
use super::state_trait::StateTrait;
use super::stopped_state::StoppedState;

#[derive(Debug, PartialEq)]
pub struct StateOperator<S: StateTrait> {
  state: S,
}

impl<S: StateTrait> StateOperator<S> {
  // Cannot use PhantomData for state in StateOperator because of this method
  pub fn get_state_name(&self) -> &'static str {
    self
      .state
      .get_state_name()
  }
}

impl StateOperator<EjectedState> {
  // no methods for the ejected state
}

impl StateOperator<RunningState> {
  pub fn skip(
    &self,
    data: &mut Data,
    delta: isize,
  ) {
    data.position = data
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> StateOperator<StoppedState> {
    StateOperator {
      state: StoppedState,
    }
  }
}

impl StateOperator<StoppedState> {
  pub fn eject(self) -> StateOperator<EjectedState> {
    StateOperator {
      state: EjectedState,
    }
  }

  pub fn reset(
    &self,
    data: &mut Data,
  ) {
    data.position = 0;
  }

  pub fn run(self) -> StateOperator<RunningState> {
    StateOperator {
      state: RunningState,
    }
  }
}

impl Default for StateOperator<StoppedState> {
  fn default() -> Self {
    Self {
      state: StoppedState,
    }
  }
}
