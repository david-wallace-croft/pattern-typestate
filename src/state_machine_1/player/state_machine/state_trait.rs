use std::fmt::Debug;

pub trait StateTrait: Clone + Copy + Debug + PartialEq {
  fn get_state_name(&self) -> &'static str;
}
