//! Fixed-point storage format.

/// Description of a signed fixed-point format.
///
/// `frac_bits` determines how many low-order bits represent the
/// fractional portion of a stored integer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Format {
    frac_bits: u32,
}

impl Format {
    pub const INTEGER: Self = Self::new(0);

    #[inline]
    pub const fn new(frac_bits: u32) -> Self {
        assert!(
            frac_bits < i64::BITS,
            "fractional bit count must be smaller than i64 width",
        );

        Self { frac_bits }
    }

    #[inline]
    pub const fn frac_bits(self) -> u32 {
        self.frac_bits
    }

    #[inline]
    pub const fn integer_bits(self) -> u32 {
        i64::BITS - self.frac_bits
    }

    #[inline]
    pub const fn scale(self) -> i64 {
        1_i64 << self.frac_bits
    }

    #[inline]
    pub const fn fractional_mask(self) -> i64 {
        if self.frac_bits == 0 {
            0
        } else {
            self.scale() - 1
        }
    }

    #[inline]
    pub const fn encode_integer(self, value: i64) -> Option<i64> {
        let encoded = (value as i128) << self.frac_bits;

        if encoded < i64::MIN as i128 || encoded > i64::MAX as i128 {
            None
        } else {
            Some(encoded as i64)
        }
    }

    #[inline]
    pub const fn decode_integer(self, raw: i64) -> i64 {
        raw >> self.frac_bits
    }
}

#[cfg(test)]
mod tests {
    use super::Format;

    #[test]
    fn integer_format_has_unit_scale() {
        let format = Format::INTEGER;

        assert_eq!(format.frac_bits(), 0);
        assert_eq!(format.scale(), 1);
        assert_eq!(format.fractional_mask(), 0);
    }

    #[test]
    fn computes_binary_scale() {
        let format = Format::new(8);

        assert_eq!(format.frac_bits(), 8);
        assert_eq!(format.integer_bits(), 56);
        assert_eq!(format.scale(), 256);
        assert_eq!(format.fractional_mask(), 255);
    }

    #[test]
    fn encodes_and_decodes_integer_values() {
        let format = Format::new(4);
        let raw = format.encode_integer(7).unwrap();

        assert_eq!(raw, 112);
        assert_eq!(format.decode_integer(raw), 7);
    }

    #[test]
    fn reports_encoding_overflow() {
        let format = Format::new(32);

        assert_eq!(format.encode_integer(i64::MAX), None);
    }
}
