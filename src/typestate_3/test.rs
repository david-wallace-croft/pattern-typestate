use super::ejected::EjectedState;
use super::request::Request::{self, *};
use super::running::RunningState;
use super::stopped::StoppedState;
use super::typestate::Typestate;
use crate::typestate_3::state_operator::StateOperator;

#[test]
pub fn test0() {
  let input_output_pairs: Vec<(Request, Typestate)> = vec![
    (
      Skip(1),
      Typestate::Stopped(StateOperator::<StoppedState>::new(0)),
    ),
    (
      Stop,
      Typestate::Stopped(StateOperator::<StoppedState>::new(0)),
    ),
    (
      Run,
      Typestate::Running(StateOperator::<RunningState>::new(0)),
    ),
    (
      Skip(2),
      Typestate::Running(StateOperator::<RunningState>::new(2)),
    ),
    (
      Skip(-1),
      Typestate::Running(StateOperator::<RunningState>::new(1)),
    ),
    (
      Eject,
      Typestate::Running(StateOperator::<RunningState>::new(1)),
    ),
    (
      Reset,
      Typestate::Running(StateOperator::<RunningState>::new(1)),
    ),
    (
      Run,
      Typestate::Running(StateOperator::<RunningState>::new(1)),
    ),
    (
      Stop,
      Typestate::Stopped(StateOperator::<StoppedState>::new(1)),
    ),
    (
      Reset,
      Typestate::Stopped(StateOperator::<StoppedState>::new(0)),
    ),
    (
      Skip(1),
      Typestate::Stopped(StateOperator::<StoppedState>::new(0)),
    ),
    (
      Stop,
      Typestate::Stopped(StateOperator::<StoppedState>::new(0)),
    ),
    (
      Run,
      Typestate::Running(StateOperator::<RunningState>::new(0)),
    ),
    (
      Skip(1),
      Typestate::Running(StateOperator::<RunningState>::new(1)),
    ),
    (
      Stop,
      Typestate::Stopped(StateOperator::<StoppedState>::new(1)),
    ),
    (
      Eject,
      Typestate::Ejected(StateOperator::<EjectedState>::new(1)),
    ),
    (
      Reset,
      Typestate::Ejected(StateOperator::<EjectedState>::new(1)),
    ),
    (
      Run,
      Typestate::Ejected(StateOperator::<EjectedState>::new(1)),
    ),
    (
      Skip(2),
      Typestate::Ejected(StateOperator::<EjectedState>::new(1)),
    ),
    (
      Stop,
      Typestate::Ejected(StateOperator::<EjectedState>::new(1)),
    ),
  ];

  let mut type_state = Typestate::default();

  assert_eq!(
    type_state,
    Typestate::Stopped(StateOperator::<StoppedState>::new(0))
  );

  for (request, expected_type_state) in input_output_pairs {
    type_state = type_state.transit(&request);

    assert_eq!(type_state, expected_type_state);
  }
}
