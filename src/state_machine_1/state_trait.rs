use std::fmt::Display;

pub trait StateTrait: Display {
  fn get_position(&self) -> usize;
}
