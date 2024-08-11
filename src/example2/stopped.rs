use super::ejected::EjectedData;
use super::request::Request;
use super::running::RunningData;
use super::typestate::Typestate;

#[derive(Debug, PartialEq)]
pub struct StoppedData {
  pub position: usize,
}

impl StoppedData {
  pub fn get_position(&self) -> usize {
    self.position
  }

  pub fn eject(self) -> EjectedData {
    EjectedData::new(self.position)
  }

  pub fn new(position: usize) -> Self {
    Self {
      position,
    }
  }

  pub fn reset(&mut self) {
    self.position = 0;
  }

  pub fn run(self) -> RunningData {
    RunningData::new(self.position)
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
