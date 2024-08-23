//==============================================================================
//! An example of a fluent constructor that uses the typestate pattern.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-08-23
//! - Updated: 2024-08-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::super::constructor_creator::ConstructorCreator;
use super::super::widget::Widget;

const DEFAULT_HEIGHT: usize = 11;
const DEFAULT_OFFSET: isize = 10;

//==============================================================================

impl ConstructorCreator<WidgetConstructor> for Widget {
  // The public ConstructorCreator trait provides indirect access to the fields
  fn constructor() -> WidgetConstructor {
    // The constructor submodule has direct access to the private fields
    let widget = Widget {
      height: Default::default(),
      offset: Default::default(),
    };

    WidgetConstructor::new(widget)
  }
}

//==============================================================================

pub struct WidgetConstructor {
  widget: Widget,
}

impl WidgetConstructor {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> Widget {
    self
      .height_default()
      .construct()
  }

  // The static constructor is only accessible to this module
  fn new(widget: Widget) -> Self {
    Self {
      widget,
    }
  }

  pub fn height(
    mut self,
    height: usize,
  ) -> WidgetConstructorOffset {
    self
      .widget
      .height = height;

    WidgetConstructorOffset::new(self.widget)
  }

  /// Use the character class-specific default value for height
  pub fn height_default(self) -> WidgetConstructorOffset {
    self.height(DEFAULT_HEIGHT)
  }
}

//==============================================================================

pub struct WidgetConstructorOffset {
  widget: Widget,
}

impl WidgetConstructorOffset {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> Widget {
    self.offset_default()
  }

  // The static constructor is only accessible to this module
  fn new(widget: Widget) -> Self {
    Self {
      widget,
    }
  }

  pub fn offset(
    mut self,
    offset: isize,
  ) -> Widget {
    self
      .widget
      .offset = offset;

    self.widget
  }

  /// Use the character class-specific default value for offset
  pub fn offset_default(self) -> Widget {
    self.offset(DEFAULT_OFFSET)
  }
}
