use super::constructor_creator::ConstructorCreator;
use crate::fluent_constructor_1::widget::Widget;

#[test]
fn test_constructor_0() {
  let actual = Widget::constructor()
    .height(10)
    .offset(12);

  assert_eq!(actual.height(), 10);

  assert_eq!(actual.offset(), 12);
}
