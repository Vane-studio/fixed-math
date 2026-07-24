//! Fixed-point numeric value.

use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::format::Format;
use super::rounding::RoundingMode;
use crate::error::{Error, Result};

/// Signed fixed-point value backed by an `i64`.
///
/// The raw integer is interpreted using an external [`Format`]:
///
/// ```text
/// real value = raw / 2^frac_bits
/// ```
///
/// Keeping the format outside the value allows the prototype to evolve
/// without making every number carry duplicate format metadata.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed {
    raw: i64,
}

impl Fixed {
    pub const ZERO: Self = Self::from_raw(0);
    pub const ONE: Self = Self::from_raw(1);

    pub const MIN: Self = Self::from_raw(i64::MIN);
    pub const MAX: Self = Self::from_raw(i64::MAX);

    #[inline]
    pub const fn new(raw: i64) -> Self {
        Self::from_raw(raw)
    }

    #[inline]
    pub const fn from_raw(raw: i64) -> Self {
        Self { raw }
    }

    #[inline]
    pub const fn raw(self) -> i64 {
        self.raw
    }

    #[inline]
    pub const fn into_raw(self) -> i64 {
        self.raw
    }

    #[inline]
    pub const fn is_zero(self) -> bool {
        self.raw == 0
    }

    #[inline]
    pub const fn is_positive(self) -> bool {
        self.raw > 0
    }

    #[inline]
    pub const fn is_negative(self) -> bool {
        self.raw < 0
    }

    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.raw.checked_add(rhs.raw) {
            Some(raw) => Some(Self::from_raw(raw)),
            None => None,
        }
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.raw.checked_sub(rhs.raw) {
            Some(raw) => Some(Self::from_raw(raw)),
            None => None,
        }
    }

    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.raw.checked_mul(rhs.raw) {
            Some(raw) => Some(Self::from_raw(raw)),
            None => None,
        }
    }

    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        match self.raw.checked_div(rhs.raw) {
            Some(raw) => Some(Self::from_raw(raw)),
            None => None,
        }
    }

    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        match self.raw.checked_neg() {
            Some(raw) => Some(Self::from_raw(raw)),
            None => None,
        }
    }

    #[inline]
    pub const fn checked_abs(self) -> Option<Self> {
        match self.raw.checked_abs() {
            Some(raw) => Some(Self::from_raw(raw)),
            None => None,
        }
    }

    /// Encodes an integer using the supplied fixed-point format.
    pub fn encode_integer(value: i64, format: Format) -> Result<Self> {
        let scale = i128::from(format.scale());

        let raw = i128::from(value)
            .checked_mul(scale)
            .ok_or(Error::Overflow)?;

        Self::from_i128(raw)
    }

    /// Decodes the integer portion of this value.
    pub fn decode_integer(self, format: Format, rounding: RoundingMode) -> Result<i64> {
        let quotient = divide_i128(i128::from(self.raw), i128::from(format.scale()), rounding)?;

        i64::try_from(quotient).map_err(|_| Error::Overflow)
    }

    /// Multiplies two values that use the same fixed-point format.
    ///
    /// For raw values `a` and `b` with scale `S`:
    ///
    /// ```text
    /// result_raw = round((a * b) / S)
    /// ```
    pub fn checked_mul_scaled(
        self,
        rhs: Self,
        format: Format,
        rounding: RoundingMode,
    ) -> Result<Self> {
        let product = i128::from(self.raw)
            .checked_mul(i128::from(rhs.raw))
            .ok_or(Error::Overflow)?;

        let raw = divide_i128(product, i128::from(format.scale()), rounding)?;

        Self::from_i128(raw)
    }

    /// Divides two values that use the same fixed-point format.
    ///
    /// For raw values `a` and `b` with scale `S`:
    ///
    /// ```text
    /// result_raw = round((a * S) / b)
    /// ```
    pub fn checked_div_scaled(
        self,
        rhs: Self,
        format: Format,
        rounding: RoundingMode,
    ) -> Result<Self> {
        if rhs.is_zero() {
            return Err(Error::DivisionByZero);
        }

        let numerator = i128::from(self.raw)
            .checked_mul(i128::from(format.scale()))
            .ok_or(Error::Overflow)?;

        let raw = divide_i128(numerator, i128::from(rhs.raw), rounding)?;

        Self::from_i128(raw)
    }

    /// Converts a value from one fractional format to another.
    pub fn rescale(self, source: Format, target: Format, rounding: RoundingMode) -> Result<Self> {
        if source == target {
            return Ok(self);
        }

        let source_bits = source.frac_bits();
        let target_bits = target.frac_bits();

        if target_bits > source_bits {
            let shift = target_bits - source_bits;

            let raw = i128::from(self.raw)
                .checked_shl(shift)
                .ok_or(Error::Overflow)?;

            Self::from_i128(raw)
        } else {
            let shift = source_bits - target_bits;
            let divisor = 1_i128.checked_shl(shift).ok_or(Error::Overflow)?;

            let raw = divide_i128(i128::from(self.raw), divisor, rounding)?;

            Self::from_i128(raw)
        }
    }

    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::from_raw(self.raw.saturating_add(rhs.raw))
    }

    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::from_raw(self.raw.saturating_sub(rhs.raw))
    }

    #[inline]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::from_raw(self.raw.saturating_mul(rhs.raw))
    }

    #[inline]
    pub const fn saturating_neg(self) -> Self {
        Self::from_raw(self.raw.saturating_neg())
    }

    #[inline]
    pub const fn saturating_abs(self) -> Self {
        Self::from_raw(self.raw.saturating_abs())
    }

    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::from_raw(self.raw.wrapping_add(rhs.raw))
    }

    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::from_raw(self.raw.wrapping_sub(rhs.raw))
    }

    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::from_raw(self.raw.wrapping_mul(rhs.raw))
    }

    #[inline]
    pub const fn wrapping_neg(self) -> Self {
        Self::from_raw(self.raw.wrapping_neg())
    }

    #[inline]
    pub const fn abs(self) -> Self {
        self.saturating_abs()
    }

    #[inline]
    pub const fn signum(self) -> Self {
        if self.raw > 0 {
            Self::ONE
        } else if self.raw < 0 {
            Self::from_raw(-1)
        } else {
            Self::ZERO
        }
    }

    #[inline]
    pub const fn min(self, other: Self) -> Self {
        if self.raw <= other.raw { self } else { other }
    }

    #[inline]
    pub const fn max(self, other: Self) -> Self {
        if self.raw >= other.raw { self } else { other }
    }

    fn from_i128(raw: i128) -> Result<Self> {
        let raw = i64::try_from(raw).map_err(|_| Error::Overflow)?;

        Ok(Self::from_raw(raw))
    }
}

fn divide_i128(numerator: i128, denominator: i128, rounding: RoundingMode) -> Result<i128> {
    if denominator == 0 {
        return Err(Error::DivisionByZero);
    }

    let quotient = numerator.checked_div(denominator).ok_or(Error::Overflow)?;

    let remainder = numerator.checked_rem(denominator).ok_or(Error::Overflow)?;

    if remainder == 0 {
        return Ok(quotient);
    }

    let same_sign = (numerator < 0) == (denominator < 0);

    match rounding {
        RoundingMode::TowardZero => Ok(quotient),

        RoundingMode::Floor => {
            if same_sign {
                Ok(quotient)
            } else {
                quotient.checked_sub(1).ok_or(Error::Overflow)
            }
        }

        RoundingMode::Ceil => {
            if same_sign {
                quotient.checked_add(1).ok_or(Error::Overflow)
            } else {
                Ok(quotient)
            }
        }

        RoundingMode::Nearest => {
            let remainder_magnitude = remainder.checked_abs().ok_or(Error::Overflow)?;

            let denominator_magnitude = denominator.checked_abs().ok_or(Error::Overflow)?;

            let doubled_remainder = remainder_magnitude.checked_mul(2).ok_or(Error::Overflow)?;

            if doubled_remainder < denominator_magnitude {
                Ok(quotient)
            } else if same_sign {
                quotient.checked_add(1).ok_or(Error::Overflow)
            } else {
                quotient.checked_sub(1).ok_or(Error::Overflow)
            }
        }
    }
}

impl From<i64> for Fixed {
    #[inline]
    fn from(value: i64) -> Self {
        Self::from_raw(value)
    }
}

impl From<i32> for Fixed {
    #[inline]
    fn from(value: i32) -> Self {
        Self::from_raw(i64::from(value))
    }
}

impl From<i16> for Fixed {
    #[inline]
    fn from(value: i16) -> Self {
        Self::from_raw(i64::from(value))
    }
}

impl From<i8> for Fixed {
    #[inline]
    fn from(value: i8) -> Self {
        Self::from_raw(i64::from(value))
    }
}

impl From<Fixed> for i64 {
    #[inline]
    fn from(value: Fixed) -> Self {
        value.into_raw()
    }
}

impl Add for Fixed {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.raw + rhs.raw)
    }
}

impl AddAssign for Fixed {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.raw += rhs.raw;
    }
}

impl Sub for Fixed {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.raw - rhs.raw)
    }
}

impl SubAssign for Fixed {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.raw -= rhs.raw;
    }
}

impl Mul for Fixed {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.raw * rhs.raw)
    }
}

impl MulAssign for Fixed {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.raw *= rhs.raw;
    }
}

impl Div for Fixed {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_raw(self.raw / rhs.raw)
    }
}

impl DivAssign for Fixed {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.raw /= rhs.raw;
    }
}

impl Neg for Fixed {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_raw(-self.raw)
    }
}

impl fmt::Display for Fixed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::Fixed;
    use crate::number::{Format, RoundingMode};

    #[test]
    fn encodes_and_decodes_integer() {
        let format = Format::new(8);

        let value = Fixed::encode_integer(3, format).unwrap();

        assert_eq!(value, Fixed::from_raw(768),);

        assert_eq!(
            value.decode_integer(format, RoundingMode::TowardZero,),
            Ok(3),
        );
    }

    #[test]
    fn multiplies_scaled_values() {
        let format = Format::new(8);

        let one_and_half = Fixed::from_raw(384);
        let two = Fixed::from_raw(512);

        assert_eq!(
            one_and_half.checked_mul_scaled(two, format, RoundingMode::Nearest,),
            Ok(Fixed::from_raw(768)),
        );
    }

    #[test]
    fn divides_scaled_values() {
        let format = Format::new(8);

        let three = Fixed::from_raw(768);
        let two = Fixed::from_raw(512);

        assert_eq!(
            three.checked_div_scaled(two, format, RoundingMode::Nearest,),
            Ok(Fixed::from_raw(384)),
        );
    }

    #[test]
    fn rounds_scaled_division() {
        let format = Format::new(4);

        let one = Fixed::from_raw(16);
        let six = Fixed::from_raw(96);

        assert_eq!(
            one.checked_div_scaled(six, format, RoundingMode::TowardZero,),
            Ok(Fixed::from_raw(2)),
        );

        assert_eq!(
            one.checked_div_scaled(six, format, RoundingMode::Nearest,),
            Ok(Fixed::from_raw(3)),
        );
    }

    #[test]
    fn rescales_values() {
        let q4 = Format::new(4);
        let q8 = Format::new(8);

        let value = Fixed::from_raw(24);

        assert_eq!(
            value.rescale(q4, q8, RoundingMode::Nearest,),
            Ok(Fixed::from_raw(384)),
        );

        assert_eq!(
            Fixed::from_raw(384).rescale(q8, q4, RoundingMode::Nearest,),
            Ok(value),
        );
    }

    #[test]
    fn reports_scaled_division_by_zero() {
        let format = Format::new(8);

        assert_eq!(
            Fixed::ONE.checked_div_scaled(Fixed::ZERO, format, RoundingMode::Nearest,),
            Err(crate::error::Error::DivisionByZero),
        );
    }
}
