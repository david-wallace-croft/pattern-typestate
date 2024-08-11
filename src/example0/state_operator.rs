use std::marker::PhantomData;

#[derive(Debug)]
pub struct StateOperator<D> {
  pub _data: PhantomData<D>,
}
