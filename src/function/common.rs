//! Shared mathematical-function configuration.

use crate::cf::EvaluationAlgorithm;
use crate::number::{Format, RoundingMode};

/// Arithmetic and convergence configuration shared by functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EvaluationConfig {
    format: Format,
    rounding: RoundingMode,
    depth: usize,
    algorithm: EvaluationAlgorithm,
}

impl EvaluationConfig {
    /// Creates a function configuration.
    ///
    /// Backward continued-fraction evaluation is used by default.
    #[inline]
    pub const fn new(format: Format, rounding: RoundingMode, depth: usize) -> Self {
        Self {
            format,
            rounding,
            depth,
            algorithm: EvaluationAlgorithm::Backward,
        }
    }

    /// Creates an integer-format configuration.
    #[inline]
    pub const fn integer(depth: usize) -> Self {
        Self::new(Format::INTEGER, RoundingMode::TowardZero, depth)
    }

    /// Creates a binary fixed-point configuration using nearest rounding.
    #[inline]
    pub const fn binary(frac_bits: u32, depth: usize) -> Self {
        Self::new(Format::new(frac_bits), RoundingMode::Nearest, depth)
    }

    #[inline]
    pub const fn format(self) -> Format {
        self.format
    }

    #[inline]
    pub const fn rounding(self) -> RoundingMode {
        self.rounding
    }

    #[inline]
    pub const fn depth(self) -> usize {
        self.depth
    }

    #[inline]
    pub const fn algorithm(self) -> EvaluationAlgorithm {
        self.algorithm
    }

    #[inline]
    pub const fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    #[inline]
    pub const fn with_rounding(mut self, rounding: RoundingMode) -> Self {
        self.rounding = rounding;
        self
    }

    #[inline]
    pub const fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    #[inline]
    pub const fn with_algorithm(mut self, algorithm: EvaluationAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    #[inline]
    pub fn set_format(&mut self, format: Format) {
        self.format = format;
    }

    #[inline]
    pub fn set_rounding(&mut self, rounding: RoundingMode) {
        self.rounding = rounding;
    }

    #[inline]
    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    #[inline]
    pub fn set_algorithm(&mut self, algorithm: EvaluationAlgorithm) {
        self.algorithm = algorithm;
    }
}

impl Default for EvaluationConfig {
    fn default() -> Self {
        Self::binary(16, 16)
    }
}

#[cfg(test)]
mod tests {
    use super::EvaluationConfig;
    use crate::cf::EvaluationAlgorithm;
    use crate::number::{Format, RoundingMode};

    #[test]
    fn creates_explicit_configuration() {
        let config = EvaluationConfig::new(Format::new(24), RoundingMode::Floor, 32);

        assert_eq!(config.format(), Format::new(24),);

        assert_eq!(config.rounding(), RoundingMode::Floor,);

        assert_eq!(config.depth(), 32,);

        assert_eq!(config.algorithm(), EvaluationAlgorithm::Backward,);
    }

    #[test]
    fn creates_integer_configuration() {
        let config = EvaluationConfig::integer(8);

        assert_eq!(config.format(), Format::INTEGER,);

        assert_eq!(config.rounding(), RoundingMode::TowardZero,);

        assert_eq!(config.depth(), 8,);

        assert_eq!(config.algorithm(), EvaluationAlgorithm::Backward,);
    }

    #[test]
    fn creates_binary_configuration() {
        let config = EvaluationConfig::binary(20, 24);

        assert_eq!(config.format(), Format::new(20),);

        assert_eq!(config.rounding(), RoundingMode::Nearest,);

        assert_eq!(config.depth(), 24,);

        assert_eq!(config.algorithm(), EvaluationAlgorithm::Backward,);
    }

    #[test]
    fn builders_replace_individual_fields() {
        let config = EvaluationConfig::default()
            .with_format(Format::new(12))
            .with_rounding(RoundingMode::Ceil)
            .with_depth(10)
            .with_algorithm(EvaluationAlgorithm::Lentz);

        assert_eq!(config.format(), Format::new(12),);

        assert_eq!(config.rounding(), RoundingMode::Ceil,);

        assert_eq!(config.depth(), 10,);

        assert_eq!(config.algorithm(), EvaluationAlgorithm::Lentz,);
    }

    #[test]
    fn builders_do_not_modify_original_value() {
        let original = EvaluationConfig::default();

        let changed = original
            .with_format(Format::new(8))
            .with_depth(40)
            .with_algorithm(EvaluationAlgorithm::Lentz);

        assert_eq!(original, EvaluationConfig::default(),);

        assert_ne!(original, changed,);
    }

    #[test]
    fn setters_replace_individual_fields() {
        let mut config = EvaluationConfig::default();

        config.set_format(Format::new(12));

        config.set_rounding(RoundingMode::Ceil);

        config.set_depth(10);

        config.set_algorithm(EvaluationAlgorithm::Lentz);

        assert_eq!(config.format(), Format::new(12),);

        assert_eq!(config.rounding(), RoundingMode::Ceil,);

        assert_eq!(config.depth(), 10,);

        assert_eq!(config.algorithm(), EvaluationAlgorithm::Lentz,);
    }

    #[test]
    fn builder_and_setter_produce_same_configuration() {
        let built = EvaluationConfig::default()
            .with_format(Format::new(28))
            .with_rounding(RoundingMode::Floor)
            .with_depth(48)
            .with_algorithm(EvaluationAlgorithm::Lentz);

        let mut mutated = EvaluationConfig::default();

        mutated.set_format(Format::new(28));

        mutated.set_rounding(RoundingMode::Floor);

        mutated.set_depth(48);

        mutated.set_algorithm(EvaluationAlgorithm::Lentz);

        assert_eq!(built, mutated,);
    }

    #[test]
    fn algorithm_can_be_switched_back_and_forth() {
        let mut config = EvaluationConfig::default();

        assert_eq!(config.algorithm(), EvaluationAlgorithm::Backward,);

        config.set_algorithm(EvaluationAlgorithm::Lentz);

        assert_eq!(config.algorithm(), EvaluationAlgorithm::Lentz,);

        config.set_algorithm(EvaluationAlgorithm::Backward);

        assert_eq!(config.algorithm(), EvaluationAlgorithm::Backward,);
    }

    #[test]
    fn zero_depth_is_preserved_for_later_validation() {
        let config = EvaluationConfig::binary(16, 0);

        assert_eq!(config.depth(), 0,);
    }

    #[test]
    fn default_uses_q16_nearest_backward_arithmetic() {
        assert_eq!(
            EvaluationConfig::default(),
            EvaluationConfig::new(Format::new(16), RoundingMode::Nearest, 16,),
        );
    }
}
