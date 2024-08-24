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
use std::marker::PhantomData;

const DEFAULT_HEIGHT: usize = 11;
const DEFAULT_OFFSET: isize = 22;
const DEFAULT_WEIGHT: f64 = 33.;

pub struct StateHeight;
pub struct StateOffset;
pub struct StateWeight;

//==============================================================================

impl ConstructorCreator<WidgetConstructor<StateHeight>> for Widget {
  // The public ConstructorCreator trait provides indirect access to the fields
  fn constructor() -> WidgetConstructor<StateHeight> {
    // The constructor submodule has direct access to the private fields
    let widget = Widget {
      height: Default::default(),
      offset: Default::default(),
      weight: Default::default(),
    };

    WidgetConstructor::<StateHeight>::new(widget)
  }
}

//==============================================================================

pub struct WidgetConstructor<S> {
  state: PhantomData<S>,
  widget: Widget,
}

impl WidgetConstructor<StateHeight> {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> Widget {
    self
      .height_default()
      .construct()
  }

  // The static constructor is only accessible to this module
  fn new(widget: Widget) -> Self {
    Self {
      state: PhantomData,
      widget,
    }
  }

  pub fn height(
    mut self,
    height: usize,
  ) -> WidgetConstructor<StateOffset> {
    self
      .widget
      .height = height;

    WidgetConstructor::<StateOffset>::new(self.widget)
  }

  /// Use the default value
  pub fn height_default(self) -> WidgetConstructor<StateOffset> {
    self.height(DEFAULT_HEIGHT)
  }
}

//==============================================================================

impl WidgetConstructor<StateOffset> {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> Widget {
    self
      .offset_default()
      .construct()
  }

  // The static constructor is only accessible to this module
  fn new(widget: Widget) -> Self {
    Self {
      state: PhantomData,
      widget,
    }
  }

  pub fn offset(
    mut self,
    offset: isize,
  ) -> WidgetConstructor<StateWeight> {
    self
      .widget
      .offset = offset;

    WidgetConstructor::<StateWeight>::new(self.widget)
  }

  /// Use the default value
  pub fn offset_default(self) -> WidgetConstructor<StateWeight> {
    self.offset(DEFAULT_OFFSET)
  }
}

//==============================================================================

impl WidgetConstructor<StateWeight> {
  /// Use the default values for the remaining fields
  pub fn construct(self) -> Widget {
    self.weight_default()
  }

  // The static constructor is only accessible to this module
  fn new(widget: Widget) -> Self {
    Self {
      state: PhantomData,
      widget,
    }
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
