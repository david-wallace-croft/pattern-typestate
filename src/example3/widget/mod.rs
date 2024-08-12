#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub struct Widget {
  a: usize,
  b: usize,
  c: Option<usize>,
  d: Option<usize>,
}

impl Widget {
  pub fn builder() -> WidgetBuilderA {
    WidgetBuilderA
  }
}

pub struct WidgetBuilderA;

impl WidgetBuilderA {
  pub fn a(
    self,
    a: usize,
  ) -> WidgetBuilderB {
    WidgetBuilderB {
      a,
    }
  }
}

pub struct WidgetBuilderB {
  a: usize,
}

impl WidgetBuilderB {
  pub fn b(
    self,
    b: usize,
  ) -> WidgetBuilderC {
    WidgetBuilderC {
      a: self.a,
      b,
    }
  }
}

pub struct WidgetBuilderC {
  a: usize,
  b: usize,
}

impl WidgetBuilderC {
  pub fn build(self) -> Widget {
    Widget {
      a: self.a,
      b: self.b,
      c: None,
      d: None,
    }
  }

  pub fn c(
    self,
    c: usize,
  ) -> WidgetBuilderD {
    WidgetBuilderD {
      a: self.a,
      b: self.b,
      c: Some(c),
    }
  }

  pub fn d(
    self,
    d: usize,
  ) -> Widget {
    Widget {
      a: self.a,
      b: self.b,
      c: None,
      d: Some(d),
    }
  }
}

pub struct WidgetBuilderD {
  a: usize,
  b: usize,
  c: Option<usize>,
}

impl WidgetBuilderD {
  pub fn build(self) -> Widget {
    Widget {
      a: self.a,
      b: self.b,
      c: self.c,
      d: None,
    }
  }

  pub fn d(
    self,
    d: usize,
  ) -> Widget {
    Widget {
      a: self.a,
      b: self.b,
      c: self.c,
      d: Some(d),
    }
  }
}
