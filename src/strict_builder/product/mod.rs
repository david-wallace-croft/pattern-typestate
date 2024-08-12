#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub struct Product {
  a: usize,
  b: usize,
  c: Option<usize>,
  d: Option<usize>,
}

impl Product {
  pub fn builder() -> StrictBuilderA {
    StrictBuilderA
  }
}

pub struct StrictBuilderA;

impl StrictBuilderA {
  pub fn a(
    self,
    a: usize,
  ) -> StrictBuilderB {
    StrictBuilderB {
      a,
    }
  }
}

pub struct StrictBuilderB {
  a: usize,
}

impl StrictBuilderB {
  pub fn b(
    self,
    b: usize,
  ) -> StrictBuilderC {
    StrictBuilderC {
      a: self.a,
      b,
    }
  }
}

pub struct StrictBuilderC {
  a: usize,
  b: usize,
}

impl StrictBuilderC {
  pub fn build(self) -> Product {
    Product {
      a: self.a,
      b: self.b,
      c: None,
      d: None,
    }
  }

  pub fn c(
    self,
    c: usize,
  ) -> StrictBuilderD {
    StrictBuilderD {
      a: self.a,
      b: self.b,
      c: Some(c),
    }
  }

  pub fn d(
    self,
    d: usize,
  ) -> Product {
    Product {
      a: self.a,
      b: self.b,
      c: None,
      d: Some(d),
    }
  }
}

pub struct StrictBuilderD {
  a: usize,
  b: usize,
  c: Option<usize>,
}

impl StrictBuilderD {
  pub fn build(self) -> Product {
    Product {
      a: self.a,
      b: self.b,
      c: self.c,
      d: None,
    }
  }

  pub fn d(
    self,
    d: usize,
  ) -> Product {
    Product {
      a: self.a,
      b: self.b,
      c: self.c,
      d: Some(d),
    }
  }
}
