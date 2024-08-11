use super::event::Event::{self, *};
use super::state_data_a::StateDataA;
use super::state_data_b::StateDataB;
use super::state_data_c::StateDataC;
use super::state_data_d::StateDataD;
use super::state_data_e::StateDataE;
use super::type_state::TypeState;

#[test]
pub fn test0() {
  let input_output_pairs: Vec<(
    Event,
    TypeState,
  )> = vec![
    (
      EventToC,
      // Cannot go from A to C
      TypeState::StateA(StateDataA::default()),
    ),
    (
      EventToB,
      // Can go from A to B
      TypeState::StateB(StateDataB::default()),
    ),
    (
      EventToA,
      // Cannot go from B to A
      TypeState::StateB(StateDataB::default()),
    ),
    (
      EventToB,
      // Can go from B to B
      TypeState::StateB(StateDataB::default()),
    ),
    (
      EventToC,
      // Can go from B to C
      TypeState::StateC(StateDataC::default()),
    ),
    (
      EventToD,
      // Can go from C to D
      TypeState::StateD(StateDataD::default()),
    ),
    (
      EventToC,
      // Can go from D to C
      TypeState::StateC(StateDataC::default()),
    ),
    (
      EventToE,
      // Can go from C to E
      TypeState::StateE(StateDataE::default()),
    ),
    (
      EventToC,
      // Cannot go from E to C
      TypeState::StateE(StateDataE::default()),
    ),
    (
      EventToD,
      // Cannot go from E to D
      TypeState::StateE(StateDataE::default()),
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
