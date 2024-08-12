use super::request::Request;
use super::state_operator::StateOperator;
use super::state_trait::StateTrait;
use super::stopped::StoppedState;
use super::typestate::Typestate;

#[derive(Debug, PartialEq)]
pub struct RunningState {
  position: usize,
}

impl StateTrait for RunningState {
  fn get_position(&self) -> usize {
    self.position
  }
}

impl StateOperator<RunningState> {
  pub fn new(position: usize) -> Self {
    StateOperator {
      state: RunningState {
        position,
      },
    }
  }

  pub fn skip(
    &mut self,
    delta: isize,
  ) {
    self
      .state
      .position = self
      .state
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> StateOperator<StoppedState> {
    StateOperator::<StoppedState>::new(
      self
        .state
        .position,
    )
  }

  pub fn transit(
    mut self,
    request: &Request,
  ) -> Typestate {
    match request {
      Request::Eject | Request::Reset | Request::Run => {
        Typestate::Running(self)
      },
      Request::Skip(delta) => {
        self.skip(*delta);

        Typestate::Running(self)
      },
      Request::Stop => Typestate::Stopped(self.stop()),
    }
  }
}
