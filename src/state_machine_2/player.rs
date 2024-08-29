use super::data::Data;
use super::request::Request;
use super::typestate::Typestate;

#[derive(Debug, PartialEq)]
pub struct Player {
  data: Data,
  typestate_option: Option<Typestate>,
}

impl Player {
  pub fn eject(&mut self) {
    self.transit(Request::Eject);
  }

  pub fn get_position(&self) -> usize {
    self
      .data
      .position
  }

  pub fn get_state(&self) -> String {
    match &self.typestate_option {
      None => "NONE".to_string(),
      Some(typestate) => format!("{typestate}"),
    }
  }

  pub fn reset(&mut self) {
    self.transit(Request::Reset);
  }

  pub fn run(&mut self) {
    self.transit(Request::Run);
  }

  pub fn skip(
    &mut self,
    delta: isize,
  ) {
    self.transit(Request::Skip(delta));
  }

  pub fn stop(&mut self) {
    self.transit(Request::Stop);
  }

  // TODO: Make this private
  pub fn transit(
    &mut self,
    request: Request,
  ) {
    // Will not compile; cannot move
    // self.typestate = self.typestate.transit(request);

    let typestate_option = self
      .typestate_option
      .take();

    let typestate_old: Typestate = typestate_option.unwrap_or_default();

    let typestate_new = match typestate_old {
      // TODO: add transit() to StateOperator
      Typestate::Ejected(state_operator) => state_operator.transit(request),
      Typestate::Running(state_operator) => {
        state_operator.transit(&mut self.data, request)
      },
      Typestate::Stopped(state_operator) => {
        state_operator.transit(&mut self.data, request)
      },
    };

    self
      .typestate_option
      .replace(typestate_new);
  }
}

impl Default for Player {
  fn default() -> Self {
    Self {
      data: Data::default(),
      typestate_option: Some(Typestate::default()),
    }
  }
}
