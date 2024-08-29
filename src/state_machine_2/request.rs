#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Request {
  Eject,       // when stopped
  Reset,       // when stopped
  Run,         // when stopped
  Skip(isize), // when running
  Stop,        // when running
}

// TODO: maybe implement Display
