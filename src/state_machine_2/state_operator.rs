use super::state_trait::StateTrait;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct StateOperator<S: StateTrait> {
  pub state: PhantomData<S>,
}
