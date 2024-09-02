use super::super::event::Event::{self, *};
use super::typestate::Typestate;

const EJECTED: &str = "EJECTED";
const RUNNING: &str = "RUNNING";
const STOPPED: &str = "STOPPED";

#[test]
pub fn test_typestate_0() {
  let test_data: Vec<(Event, &'static str, usize)> = vec![
    (Skip(1), STOPPED, 0),
    (Stop, STOPPED, 0),
    (Run, RUNNING, 0),
    (Skip(2), RUNNING, 2),
    (Skip(-1), RUNNING, 1),
    (Eject, RUNNING, 1),
    (Reset, RUNNING, 1),
    (Run, RUNNING, 1),
    (Stop, STOPPED, 1),
    (Reset, STOPPED, 0),
    (Skip(1), STOPPED, 0),
    (Stop, STOPPED, 0),
    (Run, RUNNING, 0),
    (Skip(1), RUNNING, 1),
    (Stop, STOPPED, 1),
    (Eject, EJECTED, 1),
    (Reset, EJECTED, 1),
    (Run, EJECTED, 1),
    (Skip(2), EJECTED, 1),
    (Stop, EJECTED, 1),
  ];

  let mut type_state = Typestate::default();

  assert_eq!(type_state.get_position(), 0);

  assert_eq!(type_state.get_state_name(), STOPPED);

  for (event, expected_state_name, expected_position) in test_data {
    type_state = type_state.transit(&event);

    assert_eq!(type_state.get_position(), expected_position);

    assert_eq!(type_state.get_state_name(), expected_state_name);
  }
}
