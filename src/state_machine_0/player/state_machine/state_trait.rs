use std::fmt::Display;

pub trait StateTrait: Display {
  fn get_position(&self) -> usize;

  fn get_state_name(&self) -> &'static str;
}
