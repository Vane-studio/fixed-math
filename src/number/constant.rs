//! Common numeric constants.

use super::{Fixed, Format, RoundingMode};
use crate::error::Result;

/// Additive identity.
pub const ZERO: Fixed = Fixed::ZERO;

/// Raw integer one.
///
/// This constant is only numerically equal to one when used with
/// [`Format::INTEGER`]. For scaled formats, use [`one`].
pub const ONE: Fixed = Fixed::ONE;

/// Raw integer negative one.
pub const NEGATIVE_ONE: Fixed = Fixed::from_raw(-1);

/// Raw integer two.
pub const TWO: Fixed = Fixed::from_raw(2);

/// Raw integer three.
pub const THREE: Fixed = Fixed::from_raw(3);

/// Raw integer four.
pub const FOUR: Fixed = Fixed::from_raw(4);

/// Raw integer five.
pub const FIVE: Fixed = Fixed::from_raw(5);

/// Raw integer ten.
pub const TEN: Fixed = Fixed::from_raw(10);

/// `ln(2)` encoded as Q32.
///
/// ```text
/// round(ln(2) * 2^32) = 2_977_044_471
/// ```
const LN_2_Q32: Fixed = Fixed::from_raw(2_977_044_471);

/// Creates an integer encoded using `format`.
#[inline]
pub fn integer(value: i64, format: Format) -> Result<Fixed> {
    Fixed::encode_integer(value, format)
}

/// Returns encoded zero.
#[inline]
pub const fn zero() -> Fixed {
    Fixed::ZERO
}

/// Returns encoded one.
#[inline]
pub fn one(format: Format) -> Result<Fixed> {
    integer(1, format)
}

/// Returns encoded two.
#[inline]
pub fn two(format: Format) -> Result<Fixed> {
    integer(2, format)
}

/// Returns `ln(2)` encoded in the requested format.
pub fn ln_2(format: Format, rounding: RoundingMode) -> Result<Fixed> {
    LN_2_Q32.rescale(Format::new(32), format, rounding)
}

#[cfg(test)]
mod tests {
    use super::{LN_2_Q32, integer, ln_2, one, two};
    use crate::number::{Fixed, Format, RoundingMode};

    #[test]
    fn encodes_integer_constants() {
        let format = Format::new(8);

        assert_eq!(integer(7, format,), Ok(Fixed::from_raw(1_792)),);

        assert_eq!(one(format), Ok(Fixed::from_raw(256)),);

        assert_eq!(two(format), Ok(Fixed::from_raw(512)),);
    }

    #[test]
    fn exposes_q32_ln_two() {
        assert_eq!(LN_2_Q32, Fixed::from_raw(2_977_044_471),);
    }

    #[test]
    fn rescales_ln_two_to_q16() {
        assert_eq!(
            ln_2(Format::new(16), RoundingMode::Nearest,),
            Ok(Fixed::from_raw(45_426)),
        );
    }

    #[test]
    fn rescales_ln_two_to_q8() {
        assert_eq!(
            ln_2(Format::new(8), RoundingMode::Nearest,),
            Ok(Fixed::from_raw(177)),
        );
    }

    #[test]
    fn integer_format_rounds_ln_two() {
        assert_eq!(
            ln_2(Format::INTEGER, RoundingMode::Nearest,),
            Ok(Fixed::from_raw(1)),
        );

        assert_eq!(
            ln_2(Format::INTEGER, RoundingMode::TowardZero,),
            Ok(Fixed::ZERO),
        );
    }
}
