#[derive(Debug)]
pub enum Request {
  Eject,       // when stopped
  Reset,       // when stopped
  Run,         // when stopped
  Skip(isize), // when running
  Stop,        // when running
}
