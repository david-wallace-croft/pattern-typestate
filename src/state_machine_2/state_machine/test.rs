use super::super::data::Data;
use super::super::state_machine::StateMachine;
use crate::state_machine_2::event::Event::{self, *};

#[test]
pub fn test0() {
  let input_output_data: Vec<(Event, &'static str, usize)> = vec![
    (Skip(1), "STOPPED", 0),
    (Stop, "STOPPED", 0),
    (Run, "RUNNING", 0),
    (Skip(2), "RUNNING", 2),
    (Skip(-1), "RUNNING", 1),
    (Eject, "RUNNING", 1),
    (Reset, "RUNNING", 1),
    (Run, "RUNNING", 1),
    (Stop, "STOPPED", 1),
    (Reset, "STOPPED", 0),
    (Skip(1), "STOPPED", 0),
    (Stop, "STOPPED", 0),
    (Run, "RUNNING", 0),
    (Skip(1), "RUNNING", 1),
    (Stop, "STOPPED", 1),
    (Eject, "EJECTED", 1),
    (Reset, "EJECTED", 1),
    (Run, "EJECTED", 1),
    (Skip(2), "EJECTED", 1),
    (Stop, "EJECTED", 1),
  ];

  let mut data = Data::default();

  let mut state_machine = StateMachine::default();

  assert_eq!(data.position, 0);

  assert_eq!(&state_machine.get_state(), "STOPPED");

  for (event, expected_state, expected_position) in input_output_data {
    state_machine.transit(&mut data, event);

    assert_eq!(data.position, expected_position);

    assert_eq!(&state_machine.get_state(), expected_state);
  }
}
