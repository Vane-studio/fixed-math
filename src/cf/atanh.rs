//! Inverse hyperbolic tangent continued fraction.

use super::coefficient::{Coefficient, CoefficientProvider};
use crate::error::{Error, Result};
use crate::number::{Fixed, Format, RoundingMode};

/// Coefficient provider for the inverse hyperbolic tangent.
///
/// The expansion is:
///
/// ```text
///                    x
/// atanh(x) = ---------------------
///                       x²
///             1 - ---------------
///                          4x²
///                   3 - ----------
///                              9x²
///                         5 - -----
///                              7 - ...
/// ```
///
/// In generalized continued-fraction form:
///
/// ```text
/// b₀ = 0
///
/// a₁ = x
/// b₁ = 1
///
/// aₙ = -(n - 1)² x²
/// bₙ = 2n - 1
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Atanh {
    x: Fixed,
    x_squared: Fixed,
    format: Format,
    rounding: RoundingMode,
}

impl Atanh {
    /// Creates an integer-format provider.
    pub fn new(x: Fixed) -> Result<Self> {
        Self::with_arithmetic(x, Format::INTEGER, RoundingMode::TowardZero)
    }

    /// Creates a provider using explicit fixed-point arithmetic.
    pub fn with_arithmetic(x: Fixed, format: Format, rounding: RoundingMode) -> Result<Self> {
        let one = Fixed::encode_integer(1, format)?;

        let absolute_x = x.checked_abs().ok_or(Error::Overflow)?;

        if absolute_x >= one {
            return Err(Error::Domain);
        }

        let x_squared = x.checked_mul_scaled(x, format, rounding)?;

        Ok(Self {
            x,
            x_squared,
            format,
            rounding,
        })
    }

    #[inline]
    pub const fn x(self) -> Fixed {
        self.x
    }

    #[inline]
    pub const fn x_squared(self) -> Fixed {
        self.x_squared
    }

    #[inline]
    pub const fn format(self) -> Format {
        self.format
    }

    #[inline]
    pub const fn rounding(self) -> RoundingMode {
        self.rounding
    }

    fn denominator(&self, index: usize) -> Result<Fixed> {
        let index = i64::try_from(index).map_err(|_| Error::Overflow)?;

        let odd_integer = index
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .ok_or(Error::Overflow)?;

        Fixed::encode_integer(odd_integer, self.format)
    }

    fn squared_index(&self, index: usize) -> Result<Fixed> {
        let index = i64::try_from(index).map_err(|_| Error::Overflow)?;

        let squared = index.checked_mul(index).ok_or(Error::Overflow)?;

        Fixed::encode_integer(squared, self.format)
    }

    fn later_numerator(&self, index: usize) -> Result<Fixed> {
        let multiplier = self.squared_index(index)?;

        let value = self
            .x_squared
            .checked_mul_scaled(multiplier, self.format, self.rounding)?;

        value.checked_neg().ok_or(Error::Overflow)
    }
}

impl CoefficientProvider for Atanh {
    #[inline]
    fn initial(&self) -> Fixed {
        Fixed::ZERO
    }

    fn coefficient(&self, index: usize) -> Result<Coefficient> {
        let numerator = if index == 0 {
            self.x
        } else {
            self.later_numerator(index)?
        };

        Ok(Coefficient::new(numerator, self.denominator(index)?))
    }
}

#[cfg(test)]
mod tests {
    use super::Atanh;
    use crate::cf::{Coefficient, CoefficientProvider, Evaluator};
    use crate::error::Error;
    use crate::number::{Fixed, Format, RoundingMode};
    use crate::traits::Evaluate;

    fn q16() -> Format {
        Format::new(16)
    }

    #[test]
    fn stores_input_and_square() {
        let provider =
            Atanh::with_arithmetic(Fixed::from_raw(32_768), q16(), RoundingMode::Nearest).unwrap();

        assert_eq!(provider.x(), Fixed::from_raw(32_768),);

        assert_eq!(provider.x_squared(), Fixed::from_raw(16_384),);
    }

    #[test]
    fn generates_first_coefficient() {
        let provider =
            Atanh::with_arithmetic(Fixed::from_raw(32_768), q16(), RoundingMode::Nearest).unwrap();

        assert_eq!(
            provider.coefficient(0),
            Ok(Coefficient::new(
                Fixed::from_raw(32_768),
                Fixed::from_raw(65_536),
            )),
        );
    }

    #[test]
    fn generates_later_coefficients() {
        let provider =
            Atanh::with_arithmetic(Fixed::from_raw(32_768), q16(), RoundingMode::Nearest).unwrap();

        assert_eq!(
            provider.coefficient(1),
            Ok(Coefficient::new(
                Fixed::from_raw(-16_384),
                Fixed::from_raw(196_608),
            )),
        );

        assert_eq!(
            provider.coefficient(2),
            Ok(Coefficient::new(
                Fixed::from_raw(-65_536),
                Fixed::from_raw(327_680),
            )),
        );

        assert_eq!(
            provider.coefficient(3),
            Ok(Coefficient::new(
                Fixed::from_raw(-147_456),
                Fixed::from_raw(458_752),
            )),
        );
    }

    #[test]
    fn evaluates_atanh_half() {
        let format = q16();

        let provider =
            Atanh::with_arithmetic(Fixed::from_raw(32_768), format, RoundingMode::Nearest).unwrap();

        let result = Evaluator::with_arithmetic(&provider, 16, format, RoundingMode::Nearest)
            .evaluate()
            .unwrap();

        // atanh(0.5) * 65536 ≈ 35999.45
        let difference = result.into_raw().abs_diff(35_999);

        assert!(
            difference <= 8,
            "unexpected raw value: {}",
            result.into_raw(),
        );
    }

    #[test]
    fn rejects_positive_domain_boundary() {
        assert_eq!(
            Atanh::with_arithmetic(Fixed::from_raw(65_536), q16(), RoundingMode::Nearest,),
            Err(Error::Domain),
        );
    }

    #[test]
    fn rejects_negative_domain_boundary() {
        assert_eq!(
            Atanh::with_arithmetic(Fixed::from_raw(-65_536), q16(), RoundingMode::Nearest,),
            Err(Error::Domain),
        );
    }
}
