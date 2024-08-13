use super::ejected::EjectedState;
use super::request::Request;
use super::running::RunningState;
use super::typestate::Typestate;

#[derive(Debug, PartialEq)]
pub struct StoppedState {
  pub position: usize,
}

impl StoppedState {
  pub fn get_position(&self) -> usize {
    self.position
  }

  pub fn eject(self) -> EjectedState {
    EjectedState::new(self.position)
  }

  pub fn new(position: usize) -> Self {
    Self {
      position,
    }
  }

  pub fn reset(&mut self) {
    self.position = 0;
  }

  pub fn run(self) -> RunningState {
    RunningState::new(self.position)
  }

  pub fn transit(
    mut self,
    request: &Request,
  ) -> Typestate {
    match request {
      Request::Eject => Typestate::Ejected(self.eject()),
      Request::Reset => {
        self.reset();

        Typestate::Stopped(self)
      },
      Request::Run => Typestate::Running(self.run()),
      Request::Skip(_) | Request::Stop => Typestate::Stopped(self),
    }
  }
}
