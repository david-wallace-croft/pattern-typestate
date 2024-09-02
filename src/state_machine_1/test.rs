use super::player::Player;

const EJECTED: &str = "EJECTED";
const RUNNING: &str = "RUNNING";
const STOPPED: &str = "STOPPED";

#[test]
fn test() {
  let mut player = Player::default();

  assert_eq!(player.get_state_name(), STOPPED);

  assert_eq!(player.get_position(), 0);

  player.press_run();

  assert_eq!(player.get_state_name(), RUNNING);

  player.press_skip(1);

  assert_eq!(player.get_position(), 1);

  player.press_reset();

  // Does not reset when running
  assert_eq!(player.get_position(), 1);

  player.press_stop();

  assert_eq!(player.get_state_name(), STOPPED);

  player.press_reset();

  // Does reset when stopped
  assert_eq!(player.get_position(), 0);
}
