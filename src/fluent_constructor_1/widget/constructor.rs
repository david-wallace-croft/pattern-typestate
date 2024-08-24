//==============================================================================
//! An example of a fluent constructor that uses the typestate pattern.
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

use super::super::constructor_creator::ConstructorCreator;
use super::super::widget::Widget;

const DEFAULT_HEIGHT: usize = 11;
const DEFAULT_OFFSET: isize = 22;
const DEFAULT_WEIGHT: f64 = 33.;

//==============================================================================

impl ConstructorCreator<WidgetConstructor> for Widget {
  // The public ConstructorCreator trait provides indirect access to the fields
  fn constructor() -> WidgetConstructor {
    // The constructor submodule has direct access to the private fields
    let widget = Widget {
      height: Default::default(),
      offset: Default::default(),
      weight: Default::default(),
    };

    WidgetConstructor {
      widget,
    }
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

  pub fn height(
    mut self,
    height: usize,
  ) -> WidgetConstructorOffset {
    self
      .widget
      .height = height;

    WidgetConstructorOffset {
      widget: self.widget,
    }
  }

  /// Use the default value
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
    self
      .offset_default()
      .construct()
  }

  pub fn offset(
    mut self,
    offset: isize,
  ) -> WidgetConstructorWeight {
    self
      .widget
      .offset = offset;

    WidgetConstructorWeight {
      widget: self.widget,
    }
  }

  /// Use the default value
  pub fn offset_default(self) -> WidgetConstructorWeight {
    self.offset(DEFAULT_OFFSET)
  }
}

//==============================================================================

pub struct WidgetConstructorWeight {
  widget: Widget,
}

impl WidgetConstructorWeight {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> Widget {
    self.weight_default()
  }

  pub fn weight(
    mut self,
    weight: f64,
  ) -> Widget {
    self
      .widget
      .weight = weight;

    self.widget
  }

  /// Use the default value
  pub fn weight_default(self) -> Widget {
    self.weight(DEFAULT_WEIGHT)
  }
}
