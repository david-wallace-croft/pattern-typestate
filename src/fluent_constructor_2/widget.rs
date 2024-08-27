//==============================================================================
//! The data for a Widget can only be set using the static constructor.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-08-23
//! - Updated: 2024-08-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

pub struct Widget {
  height: usize,
  offset: isize,
  weight: f64,
}

impl Widget {
  pub fn height(&self) -> usize {
    self.height
  }

  // The private fields can only be set using the static constructor
  pub fn new(
    height: usize,
    offset: isize,
    weight: f64,
  ) -> Self {
    Self {
      height,
      offset,
      weight,
    }
  }

  pub fn offset(&self) -> isize {
    self.offset
  }

  pub fn weight(&self) -> f64 {
    self.weight
  }
}
