use crate::state_machine_2::player::Player;

#[test]
fn test() {
  let mut player = Player::default();

  assert_eq!(player.get_state(), "STOPPED");

  assert_eq!(player.get_position(), 0);

  player.request_run();

  assert_eq!(player.get_state(), "RUNNING");

  player.request_skip(1);

  assert_eq!(player.get_position(), 1);

  player.request_reset();

  // Does not reset when running
  assert_eq!(player.get_position(), 1);

  player.request_stop();

  assert_eq!(player.get_state(), "STOPPED");

  player.request_reset();

  // Does reset when stopped
  assert_eq!(player.get_position(), 0);
}
