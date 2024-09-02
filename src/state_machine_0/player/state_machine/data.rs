use super::ejected_state::EjectedState;
use super::running_state::RunningState;
use super::state_trait::StateTrait;
use super::stopped_state::StoppedState;

#[derive(Debug, PartialEq)]
pub struct Data<S: StateTrait> {
  position: usize,
  state: S,
}

impl<S: StateTrait> Data<S> {
  pub fn get_position(&self) -> usize {
    self.position
  }

  // Cannot use PhantomData for state in Data because of this method
  pub fn get_state_name(&self) -> &'static str {
    self
      .state
      .get_state_name()
  }
}

impl Data<EjectedState> {
  // no methods for the ejected state
}

impl Data<RunningState> {
  pub fn skip(
    &mut self,
    delta: isize,
  ) {
    self.position = self
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> Data<StoppedState> {
    Data {
      position: self.position,
      state: StoppedState,
    }
  }
}

impl Data<StoppedState> {
  pub fn eject(self) -> Data<EjectedState> {
    Data {
      position: self.position,
      state: EjectedState,
    }
  }

  pub fn new(position: usize) -> Self {
    Self {
      position,
      state: StoppedState,
    }
  }

  pub fn reset(&mut self) {
    self.position = 0;
  }

  pub fn run(self) -> Data<RunningState> {
    Data {
      position: self.position,
      state: RunningState,
    }
  }
}
