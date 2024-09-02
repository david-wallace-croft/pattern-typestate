use self::player::Player;

mod player;

#[cfg(test)]
mod test;

pub fn example() {
  let mut player = Player::default();

  player.press_run();

  player.press_skip(1);

  player.press_stop();

  player.press_eject();
}
