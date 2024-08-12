use crate::example3::widget::{Widget, WidgetBuilderA, WidgetBuilderB};

pub mod widget;

pub fn example3() {
  // Will not compile; the fields are private
  // let widget = Widget {
  //   a: 1,
  //   b: 2,
  //   c: Some(3),
  //   d: Some(4),
  // };

  // Will not compile; no build() function until all required inputs provided
  // let widget: Widget = Widget::builder().build();

  // Will not compile; no build() function until all required inputs provided
  // let widget: Widget = Widget::builder()
  //   .a(1)
  //   .build();

  // Will not compile; cannot set the values out of order
  // let _widget: Widget = Widget::builder()
  //   .b(2)
  //   .a(1)
  //   .build();

  let _widget: Widget = Widget::builder()
    .a(1)
    .b(2)
    .build();

  let _widget: Widget = Widget::builder()
    .a(1)
    .b(2)
    .c(3)
    .build();

  // No need to call build() for the last one
  // TODO: Example where you do have to call build for the last one because
  //   a method can be called more than once.
  let _widget: Widget = Widget::builder()
    .a(1)
    .b(2)
    .c(3)
    .d(4);

  // Optional fields can be skipped
  let _widget: Widget = Widget::builder()
    .a(1)
    .b(2)
    .d(4);

  let widget_builder_a: WidgetBuilderA = Widget::builder();

  let _widget_builder_b: WidgetBuilderB = widget_builder_a.a(1);

  // Will not compile; value used after being moved
  // let widget_builder_b = widget_builder_a.a(1);

  // TODO: Example where the value chosen for an early argument restricts which
  //   arguments can be provided later, i.e., multiple build paths
}
