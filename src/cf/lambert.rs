//! Lambert continued-fraction coefficients.

use super::coefficient::{Coefficient, CoefficientProvider};
use crate::error::{Error, Result};
use crate::number::{Fixed, Format, RoundingMode};

/// Coefficient provider for Lambert's continued fraction.
///
/// The generated expansion is:
///
/// ```text
///              x
/// --------------------------------
///       x²
/// 1 + ----------------------------
///             x²
///       3 + ----------------------
///                   x²
///             5 + --------
///                   7 + ...
/// ```
///
/// This finite continued fraction approximates `tanh(x)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lambert {
    x: Fixed,
    x_squared: Fixed,
    format: Format,
}

impl Lambert {
    /// Creates a provider using integer prototype arithmetic.
    ///
    /// This constructor remains available for compatibility.
    pub fn new(x: Fixed) -> Result<Self> {
        Self::with_arithmetic(x, Format::INTEGER, RoundingMode::TowardZero)
    }

    /// Creates a provider using explicit fixed-point arithmetic.
    pub fn with_arithmetic(x: Fixed, format: Format, rounding: RoundingMode) -> Result<Self> {
        let x_squared = x.checked_mul_scaled(x, format, rounding)?;

        Ok(Self {
            x,
            x_squared,
            format,
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

    fn denominator(&self, index: usize) -> Result<Fixed> {
        let index = i64::try_from(index).map_err(|_| Error::Overflow)?;

        let odd_integer = index
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .ok_or(Error::Overflow)?;

        Fixed::encode_integer(odd_integer, self.format)
    }
}

impl CoefficientProvider for Lambert {
    #[inline]
    fn initial(&self) -> Fixed {
        Fixed::ZERO
    }

    fn coefficient(&self, index: usize) -> Result<Coefficient> {
        let numerator = if index == 0 { self.x } else { self.x_squared };

        Ok(Coefficient::new(numerator, self.denominator(index)?))
    }
}

#[cfg(test)]
mod tests {
    use super::Lambert;
    use crate::cf::coefficient::{Coefficient, CoefficientProvider};
    use crate::error::Error;
    use crate::number::{Fixed, Format, RoundingMode};

    #[test]
    fn creates_integer_provider() {
        let lambert = Lambert::new(Fixed::from_raw(4)).unwrap();

        assert_eq!(lambert.x(), Fixed::from_raw(4),);

        assert_eq!(lambert.x_squared(), Fixed::from_raw(16),);

        assert_eq!(lambert.format(), Format::INTEGER,);
    }

    #[test]
    fn creates_scaled_provider() {
        let format = Format::new(8);

        let lambert =
            Lambert::with_arithmetic(Fixed::from_raw(384), format, RoundingMode::Nearest).unwrap();

        assert_eq!(lambert.x(), Fixed::from_raw(384),);

        assert_eq!(lambert.x_squared(), Fixed::from_raw(576),);
    }

    #[test]
    fn generates_scaled_first_coefficient() {
        let format = Format::new(8);

        let lambert =
            Lambert::with_arithmetic(Fixed::from_raw(384), format, RoundingMode::Nearest).unwrap();

        assert_eq!(
            lambert.coefficient(0),
            Ok(Coefficient::new(Fixed::from_raw(384), Fixed::from_raw(256),)),
        );
    }

    #[test]
    fn generates_scaled_odd_denominators() {
        let format = Format::new(8);

        let lambert =
            Lambert::with_arithmetic(Fixed::from_raw(512), format, RoundingMode::Nearest).unwrap();

        assert_eq!(
            lambert.coefficient(1),
            Ok(Coefficient::new(
                Fixed::from_raw(1024),
                Fixed::from_raw(768),
            )),
        );

        assert_eq!(
            lambert.coefficient(2),
            Ok(Coefficient::new(
                Fixed::from_raw(1024),
                Fixed::from_raw(1280),
            )),
        );

        assert_eq!(
            lambert.coefficient(3),
            Ok(Coefficient::new(
                Fixed::from_raw(1024),
                Fixed::from_raw(1792),
            )),
        );
    }

    #[test]
    fn reports_scaled_square_overflow() {
        assert_eq!(
            Lambert::with_arithmetic(Fixed::MAX, Format::INTEGER, RoundingMode::Nearest,),
            Err(Error::Overflow),
        );
    }
}
