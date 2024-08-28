use self::ejected::EjectedState;
use self::running::RunningState;
use self::state_operator::StateOperator;
use self::stopped::StoppedState;

pub mod ejected;
pub mod request;
pub mod running;
pub mod state_operator;
pub mod state_trait;
pub mod stopped;
pub mod typestate;

#[cfg(test)]
mod test;

pub fn example() {
  let stopped = StateOperator::<StoppedState>::new(0);

  let position: usize = stopped.get_position();

  assert_eq!(position, 0);

  // method takes ownership of self
  let mut running: StateOperator<RunningState> = stopped.run();

  // Will not compile; value used after being moved
  // let mut running: StateOperator<RunningState> = stopped.run();

  // Will not compile; value used after being moved
  // let position: usize = stopped.get_position();

  // Will not compile; no method found
  // let _ = running.run();

  running.skip(1);

  let position: usize = running.get_position();

  assert_eq!(position, 1);

  let mut stopped: StateOperator<StoppedState> = running.stop();

  let position: usize = stopped.get_position();

  assert_eq!(position, 1);

  stopped.reset();

  let position: usize = stopped.get_position();

  assert_eq!(position, 0);

  let running: StateOperator<RunningState> = stopped.run();

  // Will not compile; method not found
  // let ejected = running.eject();

  let stopped: StateOperator<StoppedState> = running.stop();

  let _ejected: StateOperator<EjectedState> = stopped.eject();
}
