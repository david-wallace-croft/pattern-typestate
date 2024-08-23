//==============================================================================
//! An example of using a fluent constructor that uses the typestate pattern.
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

use self::constructor_creator::ConstructorCreator;
use self::widget::Widget;

mod constructor_creator;
mod widget;

#[cfg(test)]
mod test;

pub fn example() {
  let _widget: Widget = Widget::constructor()
    .height(10)
    .offset(10);

  // Some fields such as height have default values
  let _widget: Widget = Widget::constructor()
    .height_default()
    .offset(10);

  // Provides values only where required and uses default values for the rest
  let _widget: Widget = Widget::constructor().construct();
}
