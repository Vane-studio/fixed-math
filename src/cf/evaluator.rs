//! Generalized continued-fraction evaluator.

use super::algorithm::EvaluationAlgorithm;
use super::coefficient::{Coefficient, CoefficientProvider};
use super::model::ContinuedFraction;
use crate::error::{Error, Result};
use crate::number::{Fixed, Format, RoundingMode};
use crate::traits::evaluate::Evaluate;

/// Evaluates a finite generalized continued fraction.
///
/// The evaluated structure is:
///
/// ```text
/// b₀ + a₁ / (b₁ + a₂ / (b₂ + ... + aₙ / bₙ))
/// ```
///
/// Provider index zero represents `(a₁, b₁)`.
#[derive(Clone, Copy, Debug)]
pub struct Evaluator<'a, P> {
    provider: &'a P,
    fraction: ContinuedFraction,
    format: Format,
    rounding: RoundingMode,
    algorithm: EvaluationAlgorithm,
}

impl<'a, P> Evaluator<'a, P>
where
    P: CoefficientProvider,
{
    /// Creates an evaluator using integer arithmetic and backward
    /// evaluation.
    #[inline]
    pub const fn new(provider: &'a P, depth: usize) -> Self {
        Self {
            provider,
            fraction: ContinuedFraction::new(depth),
            format: Format::INTEGER,
            rounding: RoundingMode::TowardZero,
            algorithm: EvaluationAlgorithm::Backward,
        }
    }

    /// Creates an evaluator using explicit fixed-point arithmetic.
    #[inline]
    pub const fn with_arithmetic(
        provider: &'a P,
        depth: usize,
        format: Format,
        rounding: RoundingMode,
    ) -> Self {
        Self {
            provider,
            fraction: ContinuedFraction::new(depth),
            format,
            rounding,
            algorithm: EvaluationAlgorithm::Backward,
        }
    }

    /// Creates a fully configured evaluator.
    #[inline]
    pub const fn configured(
        provider: &'a P,
        fraction: ContinuedFraction,
        format: Format,
        rounding: RoundingMode,
        algorithm: EvaluationAlgorithm,
    ) -> Self {
        Self {
            provider,
            fraction,
            format,
            rounding,
            algorithm,
        }
    }

    #[inline]
    pub const fn provider(&self) -> &'a P {
        self.provider
    }

    #[inline]
    pub const fn fraction(&self) -> ContinuedFraction {
        self.fraction
    }

    #[inline]
    pub const fn depth(&self) -> usize {
        self.fraction.depth()
    }

    #[inline]
    pub const fn format(&self) -> Format {
        self.format
    }

    #[inline]
    pub const fn rounding(&self) -> RoundingMode {
        self.rounding
    }

    #[inline]
    pub const fn algorithm(&self) -> EvaluationAlgorithm {
        self.algorithm
    }

    #[inline]
    pub const fn with_fraction(mut self, fraction: ContinuedFraction) -> Self {
        self.fraction = fraction;
        self
    }

    #[inline]
    pub const fn with_depth(mut self, depth: usize) -> Self {
        self.fraction = self.fraction.with_depth(depth);
        self
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
    pub const fn with_algorithm(mut self, algorithm: EvaluationAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    #[inline]
    pub fn set_fraction(&mut self, fraction: ContinuedFraction) {
        self.fraction = fraction;
    }

    #[inline]
    pub fn set_depth(&mut self, depth: usize) {
        self.fraction = self.fraction.with_depth(depth);
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
    pub fn set_algorithm(&mut self, algorithm: EvaluationAlgorithm) {
        self.algorithm = algorithm;
    }

    #[inline]
    fn coefficient(&self, index: usize) -> Result<Coefficient> {
        self.provider.coefficient(index)
    }

    #[inline]
    fn divide(&self, numerator: Fixed, denominator: Fixed) -> Result<Fixed> {
        numerator.checked_div_scaled(denominator, self.format, self.rounding)
    }

    #[inline]
    fn multiply(&self, left: Fixed, right: Fixed) -> Result<Fixed> {
        left.checked_mul_scaled(right, self.format, self.rounding)
    }

    #[inline]
    fn encoded_one(&self) -> Result<Fixed> {
        Fixed::encode_integer(1, self.format)
    }

    #[inline]
    fn tiny(&self) -> Fixed {
        Fixed::from_raw(1)
    }

    #[inline]
    fn protect_zero(&self, value: Fixed) -> Fixed {
        if value.is_zero() { self.tiny() } else { value }
    }

    fn evaluate_backward(&self) -> Result<Fixed> {
        let last_index = self.depth() - 1;
        let last = self.coefficient(last_index)?;

        if last.denominator().is_zero() {
            return Err(Error::DivisionByZero);
        }

        let mut tail = last.denominator();

        for index in (0..last_index).rev() {
            let current = self.coefficient(index)?;
            let next = self.coefficient(index + 1)?;

            let quotient = self.divide(next.numerator(), tail)?;

            tail = current
                .denominator()
                .checked_add(quotient)
                .ok_or(Error::Overflow)?;
        }

        let first = self.coefficient(0)?;

        let quotient = self.divide(first.numerator(), tail)?;

        self.provider
            .initial()
            .checked_add(quotient)
            .ok_or(Error::Overflow)
    }

    fn evaluate_lentz(&self) -> Result<Fixed> {
        let one = self.encoded_one()?;
        let tiny = self.tiny();

        let initial = self.provider.initial();

        let mut result = if initial.is_zero() { tiny } else { initial };

        let mut c = result;
        let mut d = Fixed::ZERO;

        for index in 0..self.depth() {
            let coefficient = self.coefficient(index)?;
            let numerator = coefficient.numerator();
            let denominator = coefficient.denominator();

            let numerator_times_d = self.multiply(numerator, d)?;

            d = denominator
                .checked_add(numerator_times_d)
                .ok_or(Error::Overflow)?;

            d = self.protect_zero(d);

            let numerator_over_c = self.divide(numerator, self.protect_zero(c))?;

            c = denominator
                .checked_add(numerator_over_c)
                .ok_or(Error::Overflow)?;

            c = self.protect_zero(c);

            d = self.divide(one, d)?;

            let delta = self.multiply(c, d)?;

            result = self.multiply(result, delta)?;
        }

        Ok(result)
    }
}

impl<P> Evaluate<Result<Fixed>> for Evaluator<'_, P>
where
    P: CoefficientProvider,
{
    fn evaluate(&self) -> Result<Fixed> {
        self.fraction.validate()?;

        match self.algorithm {
            EvaluationAlgorithm::Backward => self.evaluate_backward(),

            EvaluationAlgorithm::Lentz => self.evaluate_lentz(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Evaluator;
    use crate::cf::{Coefficient, CoefficientProvider, ContinuedFraction, EvaluationAlgorithm};
    use crate::error::{Error, Result};
    use crate::number::{Fixed, Format, RoundingMode};
    use crate::traits::Evaluate;

    struct TestProvider {
        initial: Fixed,
        coefficients: Vec<Coefficient>,
    }

    impl CoefficientProvider for TestProvider {
        fn initial(&self) -> Fixed {
            self.initial
        }

        fn coefficient(&self, index: usize) -> Result<Coefficient> {
            self.coefficients
                .get(index)
                .copied()
                .ok_or(Error::InvalidDepth)
        }
    }

    fn q16(value: i64) -> Fixed {
        Fixed::from_raw(value * 65_536)
    }

    fn assert_close(actual: Fixed, expected: Fixed, tolerance: u64) {
        let difference = actual.into_raw().abs_diff(expected.into_raw());

        assert!(
            difference <= tolerance,
            "actual {}, expected {}, tolerance {}",
            actual.into_raw(),
            expected.into_raw(),
            tolerance,
        );
    }

    #[test]
    fn backward_evaluates_integer_fraction() {
        let provider = TestProvider {
            initial: Fixed::from_raw(1),
            coefficients: vec![
                Coefficient::new(Fixed::from_raw(6), Fixed::from_raw(2)),
                Coefficient::new(Fixed::from_raw(4), Fixed::from_raw(2)),
            ],
        };

        let evaluator = Evaluator::new(&provider, 2);

        assert_eq!(evaluator.evaluate(), Ok(Fixed::from_raw(2)),);
    }

    #[test]
    fn backward_evaluates_scaled_fraction() {
        let format = Format::new(16);

        let provider = TestProvider {
            initial: q16(1),
            coefficients: vec![
                Coefficient::new(q16(6), q16(2)),
                Coefficient::new(q16(4), q16(2)),
            ],
        };

        let evaluator = Evaluator::with_arithmetic(&provider, 2, format, RoundingMode::Nearest);

        assert_eq!(evaluator.evaluate(), Ok(Fixed::from_raw(163_840)),);
    }

    #[test]
    fn lentz_evaluates_scaled_fraction() {
        let format = Format::new(16);

        let provider = TestProvider {
            initial: q16(1),
            coefficients: vec![
                Coefficient::new(q16(6), q16(2)),
                Coefficient::new(q16(4), q16(2)),
            ],
        };

        let evaluator = Evaluator::with_arithmetic(&provider, 2, format, RoundingMode::Nearest)
            .with_algorithm(EvaluationAlgorithm::Lentz);

        assert_close(evaluator.evaluate().unwrap(), Fixed::from_raw(163_840), 4);
    }

    #[test]
    fn algorithms_produce_similar_results() {
        let format = Format::new(16);

        let provider = TestProvider {
            initial: q16(1),
            coefficients: vec![
                Coefficient::new(q16(2), q16(3)),
                Coefficient::new(q16(4), q16(5)),
                Coefficient::new(q16(6), q16(7)),
                Coefficient::new(q16(8), q16(9)),
            ],
        };

        let backward = Evaluator::with_arithmetic(&provider, 4, format, RoundingMode::Nearest)
            .with_algorithm(EvaluationAlgorithm::Backward)
            .evaluate()
            .unwrap();

        let lentz = Evaluator::with_arithmetic(&provider, 4, format, RoundingMode::Nearest)
            .with_algorithm(EvaluationAlgorithm::Lentz)
            .evaluate()
            .unwrap();

        assert_close(lentz, backward, 8);
    }

    #[test]
    fn builder_methods_replace_fields() {
        let provider = TestProvider {
            initial: Fixed::ONE,
            coefficients: vec![Coefficient::new(Fixed::ONE, Fixed::ONE)],
        };

        let evaluator = Evaluator::new(&provider, 1)
            .with_fraction(ContinuedFraction::new(3))
            .with_depth(2)
            .with_format(Format::new(16))
            .with_rounding(RoundingMode::Nearest)
            .with_algorithm(EvaluationAlgorithm::Lentz);

        assert_eq!(evaluator.depth(), 2,);

        assert_eq!(evaluator.format(), Format::new(16),);

        assert_eq!(evaluator.rounding(), RoundingMode::Nearest,);

        assert_eq!(evaluator.algorithm(), EvaluationAlgorithm::Lentz,);
    }

    #[test]
    fn setter_methods_replace_fields() {
        let provider = TestProvider {
            initial: Fixed::ONE,
            coefficients: vec![Coefficient::new(Fixed::ONE, Fixed::ONE)],
        };

        let mut evaluator = Evaluator::new(&provider, 1);

        evaluator.set_fraction(ContinuedFraction::new(4));

        evaluator.set_depth(2);

        evaluator.set_format(Format::new(16));

        evaluator.set_rounding(RoundingMode::Nearest);

        evaluator.set_algorithm(EvaluationAlgorithm::Lentz);

        assert_eq!(evaluator.fraction(), ContinuedFraction::new(2),);

        assert_eq!(evaluator.format(), Format::new(16),);

        assert_eq!(evaluator.rounding(), RoundingMode::Nearest,);

        assert_eq!(evaluator.algorithm(), EvaluationAlgorithm::Lentz,);
    }

    #[test]
    fn rejects_zero_depth_for_all_algorithms() {
        let provider = TestProvider {
            initial: Fixed::ZERO,
            coefficients: Vec::new(),
        };

        let backward = Evaluator::new(&provider, 0);

        let lentz = Evaluator::new(&provider, 0).with_algorithm(EvaluationAlgorithm::Lentz);

        assert_eq!(backward.evaluate(), Err(Error::InvalidDepth),);

        assert_eq!(lentz.evaluate(), Err(Error::InvalidDepth),);
    }

    #[test]
    fn rejects_missing_coefficient() {
        let provider = TestProvider {
            initial: Fixed::ZERO,
            coefficients: vec![Coefficient::new(Fixed::ONE, Fixed::ONE)],
        };

        let evaluator = Evaluator::new(&provider, 2);

        assert_eq!(evaluator.evaluate(), Err(Error::InvalidDepth),);
    }

    #[test]
    fn backward_reports_zero_terminal_denominator() {
        let provider = TestProvider {
            initial: Fixed::ZERO,
            coefficients: vec![Coefficient::new(Fixed::ONE, Fixed::ZERO)],
        };

        let evaluator = Evaluator::new(&provider, 1);

        assert_eq!(evaluator.evaluate(), Err(Error::DivisionByZero),);
    }
}
