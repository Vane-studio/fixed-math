//! Continued-fraction evaluation algorithms.

/// Algorithm used to evaluate a generalized continued fraction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EvaluationAlgorithm {
    /// Evaluate the finite fraction from the deepest coefficient
    /// toward the initial term.
    ///
    /// This method is simple and deterministic, but intermediate
    /// values may become large for deep fractions.
    #[default]
    Backward,

    /// Evaluate using the modified Lentz algorithm.
    ///
    /// This method is designed for deeper continued fractions and
    /// avoids explicitly constructing a large backward recurrence.
    Lentz,
}
