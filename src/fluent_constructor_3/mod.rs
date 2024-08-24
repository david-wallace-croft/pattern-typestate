//==============================================================================
//! An example of using a fluent constructor that uses the typestate pattern.
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

use self::constructor_creator::ConstructorCreator;
use self::widget::Widget;
use crate::fluent_constructor_3::widget::constructor::WidgetConstructorWeight;

mod constructor_creator;
mod widget;

#[cfg(test)]
mod test;

pub fn example() {
  // Setting the final field returns the Widget
  let _widget: Widget = Widget::constructor()
    .height(1)
    .offset(2)
    .weight(3.);

  // Some fields such as height have default values
  let _widget: Widget = Widget::constructor()
    .height_default()
    .offset(2)
    .weight(3.);

  // Uses the default values for all fields after the first
  let _widget: Widget = Widget::constructor()
    .height(1)
    .construct();

  // Uses the default values for all fields
  let _widget: Widget = Widget::constructor().construct();

  // Since the final field is not set, returns a constructor fragment
  let widget_constructor_weight: WidgetConstructorWeight =
    Widget::constructor()
      .height(1)
      .offset(2);

  // When the final field is set, the Widget is constructed
  let _widget: Widget = widget_constructor_weight.weight(3.);

  // The constructor fragment can be reused to return a different widget
  let _widget: Widget = widget_constructor_weight.weight(33.);
}
