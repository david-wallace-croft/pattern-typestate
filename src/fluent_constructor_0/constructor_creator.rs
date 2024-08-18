pub trait ConstructorCreator<T> {
  /// Creates a fluent constructor that uses the typestate pattern
  fn constructor() -> T;
}
