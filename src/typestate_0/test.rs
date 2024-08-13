use super::event::Event::{self, *};
use super::state_data_a::StateDataA;
use super::state_data_b::StateDataB;
use super::state_data_c::StateDataC;
use super::state_data_d::StateDataD;
use super::state_data_e::StateDataE;
use super::typestate::Typestate;

#[test]
pub fn test0() {
  let input_output_pairs: Vec<(
    Event,
    Typestate,
  )> = vec![
    (
      EventToC,
      // Cannot go from A to C
      Typestate::StateA(StateDataA::default()),
    ),
    (
      EventToB,
      // Can go from A to B
      Typestate::StateB(StateDataB::default()),
    ),
    (
      EventToB,
      // Can go from B to B
      Typestate::StateB(StateDataB::default()),
    ),
    (
      EventToC,
      // Can go from B to C
      Typestate::StateC(StateDataC::default()),
    ),
    (
      EventToD,
      // Can go from C to D
      Typestate::StateD(StateDataD::default()),
    ),
    (
      EventToC,
      // Can go from D to C
      Typestate::StateC(StateDataC::default()),
    ),
    (
      EventToE,
      // Can go from C to E
      Typestate::StateE(StateDataE::default()),
    ),
    (
      EventToC,
      // Cannot go from E to C
      Typestate::StateE(StateDataE::default()),
    ),
    (
      EventToD,
      // Cannot go from E to D
      Typestate::StateE(StateDataE::default()),
    ),
  ];

  let mut type_state = Typestate::default();

  assert_eq!(
    type_state,
    Typestate::StateA(StateDataA::default()),
  );

  for (event, expected_type_state) in input_output_pairs {
    type_state = type_state.transit(&event);

    assert_eq!(
      type_state,
      expected_type_state
    );
  }
}
