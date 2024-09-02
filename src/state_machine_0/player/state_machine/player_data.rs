use super::super::state_machine::ejected::EjectedState;
use super::super::state_machine::running::RunningState;
use super::super::state_machine::state_trait::StateTrait;
use super::super::state_machine::stopped::StoppedState;

#[derive(Debug, PartialEq)]
pub struct PlayerData<S: StateTrait> {
  position: usize,
  // Cannot use PhantomData for state because of the get_state_name() method
  state: S,
}

impl<S: StateTrait> PlayerData<S> {
  pub fn get_position(&self) -> usize {
    self.position
  }

  // Cannot use PhantomData in PlayerData for state because of this method
  pub fn get_state_name(&self) -> &'static str {
    self
      .state
      .get_state_name()
  }
}

impl PlayerData<EjectedState> {}

impl PlayerData<RunningState> {
  pub fn skip(
    &mut self,
    delta: isize,
  ) {
    self.position = self
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> PlayerData<StoppedState> {
    PlayerData {
      position: self.position,
      state: StoppedState,
    }
  }
}

impl PlayerData<StoppedState> {
  pub fn eject(self) -> PlayerData<EjectedState> {
    PlayerData {
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

  pub fn run(self) -> PlayerData<RunningState> {
    PlayerData {
      position: self.position,
      state: RunningState,
    }
  }
}
