//==============================================================================
//! An example of a fluent constructor that uses the typestate pattern.
//!
//! This example shows how a fluent constructor can be implemented for a struct
//! defined in an unrelated module or external crate. The struct provides a
//! public static constructor but no other means of setting the field values.
//!
//! The reasons the struct fields cannot be set directly could include:
//! - at least one field is private which prevents the use of a struct literal
//! - #\[non_exhaustive] prevents using a struct literal outside of its crate
//! - no public mutator (setter) methods are implemented
//!
//! The fluent constructor implementation accumulates and stores the field
//! values in its state structures as the calls to the chain methods drive the
//! constructor from one state to the next.  Internally, the fluent constructor
//! completes construction by calling the public static constructor once all of
//! the required field values have been acquired.
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

use super::constructor_creator::ConstructorCreator;
use super::widget::Widget;

const DEFAULT_HEIGHT: usize = 11;
const DEFAULT_OFFSET: isize = 22;
const DEFAULT_WEIGHT: f64 = 33.;

//==============================================================================

impl ConstructorCreator<WidgetConstructor> for Widget {
  // The public ConstructorCreator trait provides indirect access to the fields
  fn constructor() -> WidgetConstructor {
    WidgetConstructor
  }
}

//==============================================================================

pub struct WidgetConstructor;

impl WidgetConstructor {
  /// Use the default values for the remaining fields
  pub fn construct(&self) -> Widget {
    self
      .height_default()
      .construct()
  }

  // Note the use of &self instead of mut self
  pub fn height(
    &self,
    height: usize,
  ) -> WidgetConstructorOffset {
    WidgetConstructorOffset {
      height,
    }
  }

  /// Use the default value
  pub fn height_default(&self) -> WidgetConstructorOffset {
    self.height(DEFAULT_HEIGHT)
  }
}

//==============================================================================

pub struct WidgetConstructorOffset {
  height: usize,
}

impl WidgetConstructorOffset {
  /// Use the default values for the remaining fields
  pub fn construct(&self) -> Widget {
    self
      .offset_default()
      .construct()
  }

  pub fn offset(
    &self,
    offset: isize,
  ) -> WidgetConstructorWeight {
    WidgetConstructorWeight {
      height: self.height,
      offset,
    }
  }

  /// Use the default value
  pub fn offset_default(&self) -> WidgetConstructorWeight {
    self.offset(DEFAULT_OFFSET)
  }
}

//==============================================================================

pub struct WidgetConstructorWeight {
  height: usize,
  offset: isize,
}

impl WidgetConstructorWeight {
  /// Use the default values for the remaining fields
  pub fn construct(&self) -> Widget {
    self.weight_default()
  }

  pub fn weight(
    &self,
    weight: f64,
  ) -> Widget {
    Widget::new(self.height, self.offset, weight)
  }

  /// Use the default value
  pub fn weight_default(&self) -> Widget {
    self.weight(DEFAULT_WEIGHT)
  }
}
