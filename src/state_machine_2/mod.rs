use self::player::Player;

mod player;

#[cfg(test)]
mod test;

pub fn example() {
  let mut player = Player::default();

  player.request_run();

  player.request_skip(1);

  player.request_stop();

  player.request_eject();
}
