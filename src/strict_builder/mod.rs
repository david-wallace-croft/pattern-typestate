use crate::strict_builder::product::{Product, StrictBuilderA, StrictBuilderB};

pub mod product;

pub fn example_strict_builder() {
  // Will not compile; the fields are private
  // let product = Product {
  //   a: 1,
  //   b: 2,
  //   c: Some(3),
  //   d: Some(4),
  // };

  // Will not compile; no build() function until all required inputs provided
  // let product: Product = Product::builder().build();

  // Will not compile; no build() function until all required inputs provided
  // let product: Product = Product::builder()
  //   .a(1)
  //   .build();

  // Will not compile; cannot set the values out of order
  // let _product: Product = Product::builder()
  //   .b(2)
  //   .a(1)
  //   .build();

  let _product: Product = Product::builder()
    .a(1)
    .b(2)
    .build();

  let _product: Product = Product::builder()
    .a(1)
    .b(2)
    .c(3)
    .build();

  // No need to call build() for the last one
  // TODO: Example where you do have to call build for the last one because
  //   a method can be called more than once.
  let _product: Product = Product::builder()
    .a(1)
    .b(2)
    .c(3)
    .d(4);

  // Optional fields can be skipped
  let _product: Product = Product::builder()
    .a(1)
    .b(2)
    .d(4);

  let product_builder_a: StrictBuilderA = Product::builder();

  let _product_builder_b: StrictBuilderB = product_builder_a.a(1);

  // Will not compile; value used after being moved
  // let product_builder_b = product_builder_a.a(1);

  // TODO: Example where the value chosen for an early argument restricts which
  //   arguments can be provided later, i.e., multiple build paths
}
