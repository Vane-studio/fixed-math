//! Natural logarithm function.

use super::common::EvaluationConfig;
use crate::cf::{Atanh, Evaluator};
use crate::error::{Error, Result};
use crate::number::{Fixed, constant};
use crate::traits::{Evaluate, Function};

/// Fixed-point natural logarithm.
///
/// The input is normalized as:
///
/// ```text
/// x = m·2^k
/// 1 <= m < 2
/// ```
///
/// Then:
///
/// ```text
/// ln(x) = ln(m) + k·ln(2)
/// ln(m) = 2·atanh((m - 1) / (m + 1))
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ln {
    input: Fixed,
    config: EvaluationConfig,
}

impl Ln {
    /// Creates a natural logarithm using the default configuration.
    #[inline]
    pub fn new(input: Fixed) -> Self {
        Self {
            input,
            config: EvaluationConfig::default(),
        }
    }

    /// Creates a natural logarithm using an explicit configuration.
    #[inline]
    pub const fn with_config(input: Fixed, config: EvaluationConfig) -> Self {
        Self { input, config }
    }

    /// Returns the input value.
    #[inline]
    pub const fn input(&self) -> Fixed {
        self.input
    }

    /// Returns the evaluation configuration.
    #[inline]
    pub const fn config(&self) -> EvaluationConfig {
        self.config
    }

    /// Returns a new function using another input.
    #[inline]
    pub const fn with_input(mut self, input: Fixed) -> Self {
        self.input = input;
        self
    }

    /// Returns a new function using another configuration.
    #[inline]
    pub const fn with_evaluation_config(mut self, config: EvaluationConfig) -> Self {
        self.config = config;
        self
    }

    /// Replaces the input in place.
    #[inline]
    pub fn set_input(&mut self, input: Fixed) {
        self.input = input;
    }

    /// Replaces the evaluation configuration in place.
    #[inline]
    pub fn set_config(&mut self, config: EvaluationConfig) {
        self.config = config;
    }

    #[inline]
    fn one(&self) -> Result<Fixed> {
        Fixed::encode_integer(1, self.config.format())
    }

    #[inline]
    fn two(&self) -> Result<Fixed> {
        Fixed::encode_integer(2, self.config.format())
    }

    #[inline]
    fn ln_two(&self) -> Result<Fixed> {
        constant::ln_2(self.config.format(), self.config.rounding())
    }

    /// Normalizes the input into `x = m·2^k`, where `1 <= m < 2`.
    fn normalize(&self) -> Result<(Fixed, i64)> {
        if !self.input.is_positive() {
            return Err(Error::Domain);
        }

        let one = self.one()?;
        let two = self.two()?;

        let mut mantissa = self.input;
        let mut power = 0_i64;

        while mantissa.raw() >= two.raw() {
            mantissa = Fixed::from_raw(mantissa.raw() >> 1);

            power = power.checked_add(1).ok_or(Error::Overflow)?;
        }

        while mantissa.raw() < one.raw() {
            let raw = mantissa.raw().checked_shl(1).ok_or(Error::Overflow)?;

            mantissa = Fixed::from_raw(raw);

            power = power.checked_sub(1).ok_or(Error::Underflow)?;
        }

        Ok((mantissa, power))
    }

    /// Computes `(m - 1) / (m + 1)`.
    fn transformed_argument(&self, mantissa: Fixed) -> Result<Fixed> {
        let one = self.one()?;

        let numerator = mantissa.checked_sub(one).ok_or(Error::Overflow)?;

        let denominator = mantissa.checked_add(one).ok_or(Error::Overflow)?;

        numerator.checked_div_scaled(denominator, self.config.format(), self.config.rounding())
    }

    /// Evaluates `atanh(value)` through its continued fraction.
    fn atanh(&self, value: Fixed) -> Result<Fixed> {
        let provider = Atanh::with_arithmetic(value, self.config.format(), self.config.rounding())?;

        Evaluator::with_arithmetic(
            &provider,
            self.config.depth(),
            self.config.format(),
            self.config.rounding(),
        )
        .with_algorithm(self.config.algorithm())
        .evaluate()
    }

    /// Evaluates `ln(m)` for a normalized mantissa.
    fn normalized_logarithm(&self, mantissa: Fixed) -> Result<Fixed> {
        let argument = self.transformed_argument(mantissa)?;

        let result = self.atanh(argument)?;
        let two = self.two()?;

        result.checked_mul_scaled(two, self.config.format(), self.config.rounding())
    }

    /// Computes `k·ln(2)`.
    fn power_component(&self, power: i64) -> Result<Fixed> {
        if power == 0 {
            return Ok(Fixed::ZERO);
        }

        let encoded_power = Fixed::encode_integer(power, self.config.format())?;

        encoded_power.checked_mul_scaled(
            self.ln_two()?,
            self.config.format(),
            self.config.rounding(),
        )
    }

    fn evaluate_inner(&self) -> Result<Fixed> {
        if self.config.depth() == 0 {
            return Err(Error::InvalidDepth);
        }

        if !self.input.is_positive() {
            return Err(Error::Domain);
        }

        let one = self.one()?;

        if self.input == one {
            return Ok(Fixed::ZERO);
        }

        let (mantissa, power) = self.normalize()?;

        let mantissa_logarithm = self.normalized_logarithm(mantissa)?;

        let power_logarithm = self.power_component(power)?;

        mantissa_logarithm
            .checked_add(power_logarithm)
            .ok_or(Error::Overflow)
    }
}

impl Default for Ln {
    fn default() -> Self {
        Self::new(Fixed::ZERO)
    }
}

impl Evaluate<Result<Fixed>> for Ln {
    #[inline]
    fn evaluate(&self) -> Result<Fixed> {
        self.evaluate_inner()
    }
}

impl Function for Ln {
    #[inline]
    fn config(&self) -> EvaluationConfig {
        self.config
    }

    #[inline]
    fn evaluate(&self, input: Fixed) -> Result<Fixed> {
        Self::with_config(input, self.config).evaluate_inner()
    }

    #[inline]
    fn eval(&self, input: Fixed) -> Result<Fixed> {
        Self::with_config(input, self.config).evaluate_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::Ln;
    use crate::cf::EvaluationAlgorithm;
    use crate::error::Error;
    use crate::function::EvaluationConfig;
    use crate::number::{Fixed, Format, RoundingMode, constant};
    use crate::traits::{Evaluate, Function};

    const SCALE: i64 = 65_536;

    fn q16(value: i64) -> Fixed {
        Fixed::from_raw(value * SCALE)
    }

    fn q16_ratio(numerator: i64, denominator: i64) -> Fixed {
        Fixed::from_raw(numerator * SCALE / denominator)
    }

    fn assert_close(actual: Fixed, expected: Fixed, tolerance: u64) {
        let difference = actual.raw().abs_diff(expected.raw());

        assert!(
            difference <= tolerance,
            "actual raw {}, expected raw {}, difference {}, tolerance {}",
            actual.raw(),
            expected.raw(),
            difference,
            tolerance,
        );
    }

    fn config(algorithm: EvaluationAlgorithm) -> EvaluationConfig {
        EvaluationConfig::binary(16, 24).with_algorithm(algorithm)
    }

    fn evaluate_with(input: Fixed, algorithm: EvaluationAlgorithm) -> Fixed {
        Evaluate::evaluate(&Ln::with_config(input, config(algorithm))).unwrap()
    }

    fn ln_two() -> Fixed {
        constant::ln_2(Format::new(16), RoundingMode::Nearest).unwrap()
    }

    #[test]
    fn constructor_preserves_input_and_configuration() {
        let config = config(EvaluationAlgorithm::Lentz);

        let function = Ln::with_config(q16(2), config);

        assert_eq!(function.input(), q16(2),);

        assert_eq!(function.config(), config,);
    }

    #[test]
    fn input_builder_does_not_modify_original() {
        let original = Ln::new(q16(1));

        let changed = original.with_input(q16(2));

        assert_eq!(original.input(), q16(1),);

        assert_eq!(changed.input(), q16(2),);
    }

    #[test]
    fn setters_replace_input_and_configuration() {
        let mut function = Ln::new(q16(1));

        let new_config = config(EvaluationAlgorithm::Lentz);

        function.set_input(q16(2));

        function.set_config(new_config);

        assert_eq!(function.input(), q16(2),);

        assert_eq!(function.config(), new_config,);
    }

    #[test]
    fn ln_one_is_zero_for_both_algorithms() {
        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_eq!(evaluate_with(q16(1), algorithm,), Fixed::ZERO,);
        }
    }

    #[test]
    fn ln_two_matches_constant_for_both_algorithms() {
        let expected = ln_two();

        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_close(evaluate_with(q16(2), algorithm), expected, 4);
        }
    }

    #[test]
    fn ln_half_is_negative_ln_two_for_both_algorithms() {
        let expected = ln_two().checked_neg().unwrap();

        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_close(evaluate_with(q16_ratio(1, 2), algorithm), expected, 4);
        }
    }

    #[test]
    fn powers_of_two_are_exact_range_reduction_cases() {
        let ln_two = ln_two();

        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            for (input, multiplier) in [
                (q16_ratio(1, 4), -2),
                (q16_ratio(1, 2), -1),
                (q16(1), 0),
                (q16(2), 1),
                (q16(4), 2),
                (q16(8), 3),
                (q16(16), 4),
            ] {
                let expected = Fixed::from_raw(ln_two.raw() * multiplier);

                assert_close(evaluate_with(input, algorithm), expected, 8);
            }
        }
    }

    #[test]
    fn backward_and_lentz_agree_on_representative_inputs() {
        for input in [
            q16_ratio(1, 4),
            q16_ratio(1, 2),
            q16(1),
            q16_ratio(3, 2),
            q16(2),
            q16(3),
            q16(4),
            q16(16),
        ] {
            let backward = evaluate_with(input, EvaluationAlgorithm::Backward);

            let lentz = evaluate_with(input, EvaluationAlgorithm::Lentz);

            assert_close(backward, lentz, 64);
        }
    }

    #[test]
    fn logarithm_is_strictly_increasing_for_positive_inputs() {
        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            let quarter = evaluate_with(q16_ratio(1, 4), algorithm);

            let half = evaluate_with(q16_ratio(1, 2), algorithm);

            let one = evaluate_with(q16(1), algorithm);

            let two = evaluate_with(q16(2), algorithm);

            let four = evaluate_with(q16(4), algorithm);

            assert!(quarter.raw() < half.raw());

            assert!(half.raw() < one.raw());

            assert!(one.raw() < two.raw());

            assert!(two.raw() < four.raw());
        }
    }

    #[test]
    fn ln_x_and_ln_reciprocal_are_opposites() {
        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            let ln_two = evaluate_with(q16(2), algorithm);

            let ln_half = evaluate_with(q16_ratio(1, 2), algorithm);

            let sum = ln_two.checked_add(ln_half).unwrap();

            assert_close(sum, Fixed::ZERO, 8);

            let ln_four = evaluate_with(q16(4), algorithm);

            let ln_quarter = evaluate_with(q16_ratio(1, 4), algorithm);

            let sum = ln_four.checked_add(ln_quarter).unwrap();

            assert_close(sum, Fixed::ZERO, 16);
        }
    }

    #[test]
    fn ln_four_is_twice_ln_two() {
        let expected = Fixed::from_raw(ln_two().raw() * 2);

        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_close(evaluate_with(q16(4), algorithm), expected, 8);
        }
    }

    #[test]
    fn ln_sixteen_is_four_times_ln_two() {
        let expected = Fixed::from_raw(ln_two().raw() * 4);

        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_close(evaluate_with(q16(16), algorithm), expected, 16);
        }
    }

    #[test]
    fn evaluate_trait_uses_stored_input() {
        let function = Ln::with_config(q16(2), config(EvaluationAlgorithm::Backward));

        let result = Evaluate::evaluate(&function).unwrap();

        assert_close(result, ln_two(), 4);
    }

    #[test]
    fn function_evaluate_uses_supplied_input() {
        let function = Ln::with_config(q16(99), config(EvaluationAlgorithm::Backward));

        let result = Function::evaluate(&function, q16(1)).unwrap();

        assert_eq!(result, Fixed::ZERO,);
    }

    #[test]
    fn function_eval_matches_function_evaluate() {
        let function = Ln::with_config(q16(1), config(EvaluationAlgorithm::Lentz));

        let input = q16(2);

        let evaluate_result = Function::evaluate(&function, input).unwrap();

        let eval_result = Function::eval(&function, input).unwrap();

        assert_eq!(eval_result, evaluate_result,);
    }

    #[test]
    fn rejects_zero_for_both_algorithms() {
        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_eq!(
                Evaluate::evaluate(&Ln::with_config(Fixed::ZERO, config(algorithm),),),
                Err(Error::Domain),
            );
        }
    }

    #[test]
    fn rejects_negative_input_for_both_algorithms() {
        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_eq!(
                Evaluate::evaluate(&Ln::with_config(q16(-1), config(algorithm),),),
                Err(Error::Domain),
            );
        }
    }

    #[test]
    fn rejects_zero_depth_for_stored_input_evaluation() {
        let config = EvaluationConfig::binary(16, 0);

        assert_eq!(
            Evaluate::evaluate(&Ln::with_config(q16(2), config,),),
            Err(Error::InvalidDepth),
        );
    }

    #[test]
    fn rejects_zero_depth_for_function_trait_evaluation() {
        let function = Ln::with_config(q16(1), EvaluationConfig::binary(16, 0));

        assert_eq!(Function::eval(&function, q16(2),), Err(Error::InvalidDepth),);
    }

    #[test]
    fn function_trait_rejects_non_positive_supplied_input() {
        let function = Ln::with_config(q16(1), config(EvaluationAlgorithm::Backward));

        assert_eq!(
            Function::evaluate(&function, Fixed::ZERO,),
            Err(Error::Domain),
        );

        assert_eq!(Function::eval(&function, q16(-1),), Err(Error::Domain),);
    }
}
