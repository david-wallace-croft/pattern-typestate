use std::fmt::Debug;

pub trait TypestateTrait: Clone + Copy + Debug + PartialEq {
  fn get_state_name() -> &'static str;
}
