use super::data::Data;
use super::ejected_state::EjectedState;
use super::running_state::RunningState;
use super::state_trait::StateTrait;
use super::stopped_state::StoppedState;
use std::marker::PhantomData;

// Does not derive Copy to demonstrate how that can be handled by StateMachine
#[derive(Debug, PartialEq)]
pub struct Typestate<S: StateTrait> {
  data: Data,
  state: PhantomData<S>,
}

impl<S: StateTrait> Typestate<S> {
  pub fn get_position(&self) -> usize {
    self
      .data
      .position
  }

  pub fn new(data: Data) -> Self {
    Self {
      data,
      state: PhantomData,
    }
  }
}

impl Typestate<EjectedState> {
  // no state transition methods for the ejected state
}

impl Typestate<RunningState> {
  pub fn skip(
    &mut self,
    delta: isize,
  ) {
    self
      .data
      .position = self
      .data
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> Typestate<StoppedState> {
    Typestate::<StoppedState>::new(self.data)
  }
}

impl Typestate<StoppedState> {
  pub fn eject(self) -> Typestate<EjectedState> {
    Typestate::<EjectedState>::new(self.data)
  }

  pub fn reset(&mut self) {
    self
      .data
      .position = 0;
  }

  pub fn run(self) -> Typestate<RunningState> {
    Typestate::<RunningState>::new(self.data)
  }
}
