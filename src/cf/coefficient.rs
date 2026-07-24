//! Continued-fraction coefficient definitions.

use crate::error::Result;
use crate::number::fixed::Fixed;

/// Coefficients for one level of a generalized continued fraction.
///
/// A coefficient pair represents:
///
/// ```text
/// aₙ / (bₙ + ...)
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Coefficient {
    numerator: Fixed,
    denominator: Fixed,
}

impl Coefficient {
    #[inline]
    pub const fn new(numerator: Fixed, denominator: Fixed) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    #[inline]
    pub const fn numerator(self) -> Fixed {
        self.numerator
    }

    #[inline]
    pub const fn denominator(self) -> Fixed {
        self.denominator
    }

    #[inline]
    pub const fn into_parts(self) -> (Fixed, Fixed) {
        (self.numerator, self.denominator)
    }
}

/// Produces coefficients for a generalized continued fraction.
///
/// Implementations may generate coefficients algorithmically instead
/// of storing every level in memory.
pub trait CoefficientProvider {
    /// Initial denominator `b₀`.
    fn initial(&self) -> Fixed;

    /// Return the coefficient pair `(aₙ, bₙ)` for `index`.
    ///
    /// Indices begin at zero in the Rust API. Index zero therefore
    /// represents the mathematical pair `(a₁, b₁)`.
    fn coefficient(&self, index: usize) -> Result<Coefficient>;
}

#[cfg(test)]
mod tests {
    use super::Coefficient;
    use crate::number::fixed::Fixed;

    #[test]
    fn creates_coefficient_pair() {
        let coefficient = Coefficient::new(Fixed::from_raw(3), Fixed::from_raw(5));

        assert_eq!(coefficient.numerator(), Fixed::from_raw(3),);

        assert_eq!(coefficient.denominator(), Fixed::from_raw(5),);
    }

    #[test]
    fn extracts_coefficient_parts() {
        let coefficient = Coefficient::new(Fixed::from_raw(7), Fixed::from_raw(11));

        assert_eq!(
            coefficient.into_parts(),
            (Fixed::from_raw(7), Fixed::from_raw(11),),
        );
    }
}
