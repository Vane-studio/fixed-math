//! Generic evaluation trait.

/// Evaluates a value.
///
/// The return type is intentionally generic so implementations may
/// return plain values, `Result<T>`, iterators, or other evaluation
/// products.
pub trait Evaluate<Output> {
    /// Evaluate the object.
    fn evaluate(&self) -> Output;
}

#[cfg(test)]
mod tests {
    use super::Evaluate;

    struct Constant(i32);

    impl Evaluate<i32> for Constant {
        fn evaluate(&self) -> i32 {
            self.0
        }
    }

    #[test]
    fn evaluates_constant() {
        let value = Constant(42);

        assert_eq!(value.evaluate(), 42,);
    }
}
