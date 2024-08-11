use self::ejected::EjectedData;
use self::running::RunningData;
use self::stopped::StoppedData;

pub mod ejected;
pub mod request;
pub mod running;
pub mod stopped;
pub mod typestate;

#[cfg(test)]
mod test;

pub fn example2() {
  let stopped = StoppedData::new(0);

  let position: usize = stopped.get_position();

  assert_eq!(position, 0);

  // method takes ownership of self
  let mut running: RunningData = stopped.run();

  // Will not compile; value used after being moved
  // let mut running: RunningData = stopped.run();

  // Will not compile; value used after being moved
  // let position: usize = stopped.get_position();

  // Will not compile; no method found
  // let _ = running.run();

  running.skip(1);

  let position: usize = running.get_position();

  assert_eq!(position, 1);

  let mut stopped: StoppedData = running.stop();

  let position: usize = stopped.get_position();

  assert_eq!(position, 1);

  stopped.reset();

  let position: usize = stopped.get_position();

  assert_eq!(position, 0);

  let running: RunningData = stopped.run();

  // Will not compile; method not found
  // let ejected = running.eject();

  let stopped: StoppedData = running.stop();

  let _ejected: EjectedData = stopped.eject();
}
