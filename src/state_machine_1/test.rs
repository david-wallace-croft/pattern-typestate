use super::player::Player;

#[test]
fn test() {
  let mut player = Player::default();

  assert_eq!(player.get_state(), "STOPPED");

  assert_eq!(player.get_position(), 0);

  player.press_run();

  assert_eq!(player.get_state(), "RUNNING");

  player.press_skip(1);

  assert_eq!(player.get_position(), 1);

  player.press_reset();

  // Does not reset when running
  assert_eq!(player.get_position(), 1);

  player.press_stop();

  assert_eq!(player.get_state(), "STOPPED");

  player.press_reset();

  // Does reset when stopped
  assert_eq!(player.get_position(), 0);
}
