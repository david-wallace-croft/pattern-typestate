use super::event::Event::{self, *};
use super::state_data_a::StateDataA;
use super::state_data_b::StateDataB;
use super::state_data_c::StateDataC;
use super::type_state::TypeState;

#[test]
pub fn test0() {
  let input_output_pairs: Vec<(
    Event,
    TypeState,
  )> = vec![
    (
      Event0,
      TypeState::StateB(StateDataB {}),
    ),
    (
      Event0,
      TypeState::StateC(StateDataC {}),
    ),
  ];

  let mut type_state = TypeState::default();

  assert_eq!(
    type_state,
    TypeState::StateA(StateDataA {})
  );

  for (event, expected_type_state) in input_output_pairs {
    type_state = type_state.transit(&event);

    assert_eq!(
      type_state,
      expected_type_state
    );
  }
}
