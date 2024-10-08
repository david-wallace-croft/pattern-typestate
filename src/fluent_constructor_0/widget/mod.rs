//==============================================================================
//! The data for a Widget can only be set using the fluent constructor.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-08-23
//! - Updated: 2024-08-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

// The constructor submodule provides a ConstructorCreator trait implementation
mod constructor;

// The private fields can only be set in the constructor submodule
pub struct Widget {
  height: usize,
  offset: isize,
  weight: f64,
}

// Provides field accessor (getter) methods but no mutator (setter) methods
impl Widget {
  pub fn height(&self) -> usize {
    self.height
  }

  pub fn offset(&self) -> isize {
    self.offset
  }

  pub fn weight(&self) -> f64 {
    self.weight
  }
}
