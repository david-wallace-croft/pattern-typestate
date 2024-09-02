use super::super::event::Event::{self, *};
use super::typestate::Typestate;

const EJECTED: &str = "EJECTED";
const RUNNING: &str = "RUNNING";
const STOPPED: &str = "STOPPED";

#[test]
pub fn test_typestate_0() {
  let test_data: Vec<(Event, usize, &'static str)> = vec![
    (Skip(1), 0, STOPPED),
    (Run, 0, RUNNING),
    (Skip(2), 2, RUNNING),
    (Skip(-1), 1, RUNNING),
    (Eject, 1, RUNNING),
    (Reset, 1, RUNNING),
    (Run, 1, RUNNING),
    (Stop, 1, STOPPED),
    (Reset, 0, STOPPED),
    (Skip(1), 0, STOPPED),
    (Stop, 0, STOPPED),
    (Run, 0, RUNNING),
    (Skip(1), 1, RUNNING),
    (Stop, 1, STOPPED),
    (Eject, 1, EJECTED),
    (Reset, 1, EJECTED),
    (Run, 1, EJECTED),
    (Skip(2), 1, EJECTED),
    (Stop, 1, EJECTED),
  ];

  let mut type_state = Typestate::default();

  assert_eq!(type_state.get_position(), 0);

  assert_eq!(type_state.get_state_name(), STOPPED);

  for (event, expected_position, expected_state_name) in test_data {
    type_state = type_state.transit(&event);

    assert_eq!(type_state.get_position(), expected_position);

    assert_eq!(type_state.get_state_name(), expected_state_name);
  }
}
