use super::event::Event::{self, *};
use super::initialized::InitializedData;
use super::running::RunningData;
use super::state_data_e::DestroyedData;
use super::suspended::SuspendedData;
use super::typestate::Typestate;
use super::uninitialized::UninitializedData;

#[test]
pub fn test0() {
  let input_output_pairs: Vec<(
    Event,
    Typestate,
  )> = vec![
    (
      Start,
      Typestate::Uninitialized(UninitializedData::default()),
    ),
    (
      Initialize,
      Typestate::Initialized(InitializedData::default()),
    ),
    (
      Initialize,
      Typestate::Initialized(InitializedData::default()),
    ),
    (
      Start,
      Typestate::Running(RunningData::default()),
    ),
    (
      Stop,
      Typestate::Suspended(SuspendedData::default()),
    ),
    (
      Start,
      Typestate::Running(RunningData::default()),
    ),
    (
      Destroy,
      Typestate::Destroyed(DestroyedData::default()),
    ),
    (
      Start,
      Typestate::Destroyed(DestroyedData::default()),
    ),
    (
      Stop,
      Typestate::Destroyed(DestroyedData::default()),
    ),
  ];

  let mut type_state = Typestate::default();

  assert_eq!(
    type_state,
    Typestate::Uninitialized(UninitializedData::default())
  );

  for (event, expected_type_state) in input_output_pairs {
    type_state = type_state.transit(&event);

    assert_eq!(
      type_state,
      expected_type_state
    );
  }
}
