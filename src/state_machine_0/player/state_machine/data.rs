use super::super::state_machine::ejected::EjectedState;
use super::super::state_machine::running::RunningState;
use super::super::state_machine::state_trait::StateTrait;
use super::super::state_machine::stopped::StoppedState;

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

impl Data<EjectedState> {}

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