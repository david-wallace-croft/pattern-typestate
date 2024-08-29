use super::state_trait::StateTrait;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct StateOperator<S: StateTrait> {
  pub state: PhantomData<S>,
}

// TODO: might need to add transit to StateTrait as well
// impl<S: StateTrait> for StateOperator<S> {
//   pub fn transit(&mut self) ->  {
//     self.state.transit()
//   }
// }
