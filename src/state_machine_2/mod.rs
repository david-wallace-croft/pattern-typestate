use self::player::Player;

mod data;
mod event;
mod player;
mod state_machine;

#[cfg(test)]
mod test;

pub fn example() {
  let mut player = Player::default();

  player.request_run();

  // TODO
}
