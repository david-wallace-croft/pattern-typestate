use std::fmt::Debug;

pub trait StateTrait: Debug + PartialEq {
  fn get_state_name() -> &'static str;
}
