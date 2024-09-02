#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
  Eject,       // when stopped
  Reset,       // when stopped
  Run,         // when stopped
  Skip(isize), // when running
  Stop,        // when running
}
