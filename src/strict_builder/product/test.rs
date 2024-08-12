use super::*;

#[test]
pub fn test0() {
  let expected = Product {
    a: 1,
    b: 2,
    c: Some(3),
    d: Some(4),
  };

  let actual: Product = Product::builder()
    .a(1)
    .b(2)
    .c(3)
    .d(4);

  assert_eq!(actual, expected);
}
