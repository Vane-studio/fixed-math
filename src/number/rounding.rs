//! Rounding behavior.

/// Rounding strategy used when an exact fixed-point result cannot
/// be represented.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum RoundingMode {
    /// Round toward negative infinity.
    Floor,

    /// Round toward positive infinity.
    Ceil,

    /// Round toward zero.
    TowardZero,

    /// Round to the nearest integer.
    ///
    /// Exact half-way cases are rounded away from zero in the
    /// current prototype.
    #[default]
    Nearest,
}

impl RoundingMode {
    /// Divide `numerator` by `denominator` using this rounding mode.
    ///
    /// Returns `None` when the denominator is zero or the result
    /// cannot be represented by an `i64`.
    pub fn divide(self, numerator: i64, denominator: i64) -> Option<i64> {
        if denominator == 0 {
            return None;
        }

        let numerator = i128::from(numerator);
        let denominator = i128::from(denominator);

        let quotient = numerator / denominator;
        let remainder = numerator % denominator;

        let rounded = match self {
            Self::TowardZero => quotient,

            Self::Floor => {
                if remainder != 0 && signs_differ(numerator, denominator) {
                    quotient - 1
                } else {
                    quotient
                }
            }

            Self::Ceil => {
                if remainder != 0 && !signs_differ(numerator, denominator) {
                    quotient + 1
                } else {
                    quotient
                }
            }

            Self::Nearest => round_nearest(quotient, remainder, denominator),
        };

        i64::try_from(rounded).ok()
    }
}

#[inline]
fn signs_differ(left: i128, right: i128) -> bool {
    (left < 0) != (right < 0)
}

fn round_nearest(quotient: i128, remainder: i128, denominator: i128) -> i128 {
    if remainder == 0 {
        return quotient;
    }

    let doubled_remainder = remainder.abs() * 2;
    let denominator = denominator.abs();

    if doubled_remainder < denominator {
        return quotient;
    }

    if quotient < 0 || (quotient == 0 && remainder < 0) {
        quotient - 1
    } else {
        quotient + 1
    }
}

#[cfg(test)]
mod tests {
    use super::RoundingMode;

    #[test]
    fn rounds_toward_zero() {
        assert_eq!(RoundingMode::TowardZero.divide(7, 3), Some(2));
        assert_eq!(RoundingMode::TowardZero.divide(-7, 3), Some(-2));
    }

    #[test]
    fn rounds_toward_negative_infinity() {
        assert_eq!(RoundingMode::Floor.divide(7, 3), Some(2));
        assert_eq!(RoundingMode::Floor.divide(-7, 3), Some(-3));
    }

    #[test]
    fn rounds_toward_positive_infinity() {
        assert_eq!(RoundingMode::Ceil.divide(7, 3), Some(3));
        assert_eq!(RoundingMode::Ceil.divide(-7, 3), Some(-2));
    }

    #[test]
    fn rounds_to_nearest() {
        assert_eq!(RoundingMode::Nearest.divide(7, 3), Some(2));
        assert_eq!(RoundingMode::Nearest.divide(8, 3), Some(3));
    }

    #[test]
    fn rounds_half_away_from_zero() {
        assert_eq!(RoundingMode::Nearest.divide(5, 2), Some(3));
        assert_eq!(RoundingMode::Nearest.divide(-5, 2), Some(-3));
    }

    #[test]
    fn rejects_zero_denominator() {
        assert_eq!(RoundingMode::Nearest.divide(1, 0), None);
    }
}
