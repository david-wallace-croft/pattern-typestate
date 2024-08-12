use super::*;

#[test]
pub fn test0() {
  let expected = Widget {
    a: 1,
    b: 2,
    c: Some(3),
    d: Some(4),
  };

  let actual: Widget = Widget::builder()
    .a(1)
    .b(2)
    .c(3)
    .d(4);

  assert_eq!(actual, expected);
}
