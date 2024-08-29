use super::player::Player;
use super::request::Request::{self, *};

#[test]
pub fn test0() {
  let input_output_data: Vec<(Request, &'static str, usize)> = vec![
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

  let mut player = Player::default();

  assert_eq!(player.get_position(), 0);

  assert_eq!(&player.get_state(), "STOPPED");

  for (request, expected_state, expected_position) in input_output_data {
    player.transit(request);

    assert_eq!(player.get_position(), expected_position);

    assert_eq!(&player.get_state(), expected_state);
  }
}
