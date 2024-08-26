use super::constructor_creator::ConstructorCreator;
use super::widget::Widget;

#[test]
fn test_constructor_0() {
  let actual = Widget::constructor()
    .height(1)
    .offset(2)
    .weight(3.);

  assert_eq!(actual.height(), 1);

  assert_eq!(actual.offset(), 2);

  assert_eq!(actual.weight(), 3.);
}

#[test]
fn test_constructor_1() {
  let actual = Widget::constructor().construct();

  assert_eq!(actual.height(), 11);

  assert_eq!(actual.offset(), 22);

  assert_eq!(actual.weight(), 33.);
}

#[test]
fn test_constructor_2() {
  let actual = Widget::constructor()
    .height(1)
    .construct();

  assert_eq!(actual.height(), 1);

  assert_eq!(actual.offset(), 22);

  assert_eq!(actual.weight(), 33.);
}

#[test]
fn test_constructor_3() {
  let actual = Widget::constructor()
    .height_default()
    .offset(2)
    .weight(3.);

  assert_eq!(actual.height(), 11);

  assert_eq!(actual.offset(), 2);

  assert_eq!(actual.weight(), 3.);
}
