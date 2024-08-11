use super::request::Request;
use super::stopped::StoppedData;
use super::typestate::Typestate;

#[derive(Debug, PartialEq)]
pub struct RunningData {
  position: usize,
}

impl RunningData {
  pub fn get_position(&self) -> usize {
    self.position
  }

  pub fn new(position: usize) -> Self {
    Self {
      position,
    }
  }

  pub fn skip(
    &mut self,
    delta: isize,
  ) {
    self.position = self
      .position
      .saturating_add_signed(delta);
  }

  pub fn stop(self) -> StoppedData {
    StoppedData::new(self.position)
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
