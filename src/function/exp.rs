//! Exponential function.

use super::common::EvaluationConfig;
use crate::cf::{Evaluator, Lambert};
use crate::error::{Error, Result};
use crate::number::{Fixed, constant};
use crate::traits::{Evaluate, Function};

/// Fixed-point exponential function.
///
/// The implementation uses range reduction:
///
/// ```text
/// x = k·ln(2) + r
/// exp(x) = 2^k · exp(r)
/// ```
///
/// The reduced exponential is evaluated through:
///
/// ```text
/// exp(r) = (1 + tanh(r/2)) / (1 - tanh(r/2))
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Exp {
    input: Fixed,
    config: EvaluationConfig,
}

impl Exp {
    /// Creates an exponential function using the default configuration.
    #[inline]
    pub fn new(input: Fixed) -> Self {
        Self {
            input,
            config: EvaluationConfig::default(),
        }
    }

    /// Creates an exponential function using an explicit configuration.
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

    /// Returns a new function using another input value.
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

    /// Replaces the input value in place.
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
    fn ln_two(&self) -> Result<Fixed> {
        constant::ln_2(self.config.format(), self.config.rounding())
    }

    #[inline]
    fn one(&self) -> Result<Fixed> {
        Fixed::encode_integer(1, self.config.format())
    }

    #[inline]
    fn half(&self, value: Fixed) -> Result<Fixed> {
        let two = Fixed::encode_integer(2, self.config.format())?;

        value.checked_div_scaled(two, self.config.format(), self.config.rounding())
    }

    /// Computes the integer range-reduction power `k`.
    fn reduction_power(&self, ln_two: Fixed) -> Result<i64> {
        if ln_two.is_zero() {
            return Err(Error::DivisionByZero);
        }

        let quotient =
            self.input
                .checked_div_scaled(ln_two, self.config.format(), self.config.rounding())?;

        Fixed::decode_integer(quotient, self.config.format(), self.config.rounding())
    }

    /// Computes `r = x - k·ln(2)`.
    fn reduced_argument(&self, power: i64, ln_two: Fixed) -> Result<Fixed> {
        let encoded_power = Fixed::encode_integer(power, self.config.format())?;

        let multiple = encoded_power.checked_mul_scaled(
            ln_two,
            self.config.format(),
            self.config.rounding(),
        )?;

        self.input.checked_sub(multiple).ok_or(Error::Overflow)
    }

    /// Computes `tanh(value)` using Lambert's continued fraction.
    fn tanh(&self, value: Fixed) -> Result<Fixed> {
        let provider =
            Lambert::with_arithmetic(value, self.config.format(), self.config.rounding())?;

        Evaluator::with_arithmetic(
            &provider,
            self.config.depth(),
            self.config.format(),
            self.config.rounding(),
        )
        .with_algorithm(self.config.algorithm())
        .evaluate()
    }

    /// Evaluates the exponential on the reduced interval.
    fn evaluate_reduced(&self, argument: Fixed) -> Result<Fixed> {
        let one = self.one()?;
        let half_argument = self.half(argument)?;
        let tangent = self.tanh(half_argument)?;

        let numerator = one.checked_add(tangent).ok_or(Error::Overflow)?;

        let denominator = one.checked_sub(tangent).ok_or(Error::Overflow)?;

        numerator.checked_div_scaled(denominator, self.config.format(), self.config.rounding())
    }

    /// Multiplies or divides by `2^power`.
    fn scale_power_of_two(&self, value: Fixed, power: i64) -> Result<Fixed> {
        if power == 0 {
            return Ok(value);
        }

        if power > 0 {
            let shift = u32::try_from(power).map_err(|_| Error::Overflow)?;

            let raw = value.raw().checked_shl(shift).ok_or(Error::Overflow)?;

            Ok(Fixed::from_raw(raw))
        } else {
            let magnitude = power.checked_neg().ok_or(Error::Overflow)?;

            let shift = u32::try_from(magnitude).map_err(|_| Error::Underflow)?;

            if shift >= i64::BITS {
                return Ok(Fixed::ZERO);
            }

            Ok(Fixed::from_raw(value.raw() >> shift))
        }
    }

    fn evaluate_inner(&self) -> Result<Fixed> {
        if self.config.depth() == 0 {
            return Err(Error::InvalidDepth);
        }

        if self.input.is_zero() {
            return self.one();
        }

        let ln_two = self.ln_two()?;
        let power = self.reduction_power(ln_two)?;

        let reduced = self.reduced_argument(power, ln_two)?;

        let reduced_result = self.evaluate_reduced(reduced)?;

        self.scale_power_of_two(reduced_result, power)
    }
}

impl Default for Exp {
    fn default() -> Self {
        Self::new(Fixed::ZERO)
    }
}

impl Evaluate<Result<Fixed>> for Exp {
    #[inline]
    fn evaluate(&self) -> Result<Fixed> {
        self.evaluate_inner()
    }
}

impl Function for Exp {
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
    use super::Exp;
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

    fn assert_close_relative(
        actual: Fixed,
        expected: Fixed,
        absolute_tolerance: u64,
        relative_numerator: u64,
        relative_denominator: u64,
    ) {
        let difference = actual.raw().abs_diff(expected.raw());

        let magnitude = actual
            .raw()
            .unsigned_abs()
            .max(expected.raw().unsigned_abs());

        let relative_tolerance =
            magnitude.saturating_mul(relative_numerator) / relative_denominator;

        let tolerance = absolute_tolerance.max(relative_tolerance);

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
        Evaluate::evaluate(&Exp::with_config(input, config(algorithm))).unwrap()
    }

    #[test]
    fn constructor_preserves_input_and_configuration() {
        let config = config(EvaluationAlgorithm::Lentz);

        let function = Exp::with_config(q16(2), config);

        assert_eq!(function.input(), q16(2),);

        assert_eq!(function.config(), config,);
    }

    #[test]
    fn input_builder_does_not_modify_original() {
        let original = Exp::new(q16(1));

        let changed = original.with_input(q16(2));

        assert_eq!(original.input(), q16(1),);

        assert_eq!(changed.input(), q16(2),);
    }

    #[test]
    fn setters_replace_input_and_configuration() {
        let mut function = Exp::new(q16(1));

        let new_config = config(EvaluationAlgorithm::Lentz);

        function.set_input(q16(2));

        function.set_config(new_config);

        assert_eq!(function.input(), q16(2),);

        assert_eq!(function.config(), new_config,);
    }

    #[test]
    fn exp_zero_is_one_for_both_algorithms() {
        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_eq!(evaluate_with(Fixed::ZERO, algorithm,), q16(1),);
        }
    }

    #[test]
    fn exp_ln_two_is_two_for_both_algorithms() {
        let ln_two = constant::ln_2(Format::new(16), RoundingMode::Nearest).unwrap();

        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_close(evaluate_with(ln_two, algorithm), q16(2), 8);
        }
    }

    #[test]
    fn exp_negative_ln_two_is_half_for_both_algorithms() {
        let ln_two = constant::ln_2(Format::new(16), RoundingMode::Nearest).unwrap();

        let negative_ln_two = ln_two.checked_neg().unwrap();

        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            assert_close(
                evaluate_with(negative_ln_two, algorithm),
                q16_ratio(1, 2),
                8,
            );
        }
    }

    #[test]
    fn backward_and_lentz_agree_on_representative_inputs() {
        for input in [
            q16(-4),
            q16(-2),
            q16(-1),
            Fixed::ZERO,
            q16_ratio(1, 2),
            q16(1),
            q16(2),
            q16(4),
        ] {
            let backward = evaluate_with(input, EvaluationAlgorithm::Backward);

            let lentz = evaluate_with(input, EvaluationAlgorithm::Lentz);

            assert_close_relative(backward, lentz, 64, 1, 2_000);
        }
    }

    #[test]
    fn exponential_is_strictly_increasing_on_small_integer_range() {
        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            let negative_two = evaluate_with(q16(-2), algorithm);

            let negative_one = evaluate_with(q16(-1), algorithm);

            let zero = evaluate_with(Fixed::ZERO, algorithm);

            let one = evaluate_with(q16(1), algorithm);

            let two = evaluate_with(q16(2), algorithm);

            assert!(negative_two.raw() < negative_one.raw());

            assert!(negative_one.raw() < zero.raw());

            assert!(zero.raw() < one.raw());

            assert!(one.raw() < two.raw());
        }
    }

    #[test]
    fn exp_x_times_exp_negative_x_is_approximately_one() {
        let format = Format::new(16);

        for algorithm in [EvaluationAlgorithm::Backward, EvaluationAlgorithm::Lentz] {
            for input in [q16_ratio(1, 2), q16(1), q16(2)] {
                let negative = input.checked_neg().unwrap();

                let positive_result = evaluate_with(input, algorithm);

                let negative_result = evaluate_with(negative, algorithm);

                let product = positive_result
                    .checked_mul_scaled(negative_result, format, RoundingMode::Nearest)
                    .unwrap();

                assert_close(product, q16(1), 192);
            }
        }
    }

    #[test]
    fn evaluate_trait_uses_stored_input() {
        let function = Exp::with_config(q16(1), config(EvaluationAlgorithm::Backward));

        let result = Evaluate::evaluate(&function).unwrap();

        assert!(result.raw() > q16(2).raw(),);

        assert!(result.raw() < q16(3).raw(),);
    }

    #[test]
    fn function_evaluate_uses_supplied_input() {
        let function = Exp::with_config(q16(99), config(EvaluationAlgorithm::Backward));

        let result = Function::evaluate(&function, Fixed::ZERO).unwrap();

        assert_eq!(result, q16(1),);
    }

    #[test]
    fn function_eval_matches_function_evaluate() {
        let function = Exp::with_config(Fixed::ZERO, config(EvaluationAlgorithm::Lentz));

        let input = q16(1);

        let evaluate_result = Function::evaluate(&function, input).unwrap();

        let eval_result = Function::eval(&function, input).unwrap();

        assert_eq!(eval_result, evaluate_result,);
    }

    #[test]
    fn function_trait_preserves_algorithm_selection() {
        let backward_function =
            Exp::with_config(Fixed::ZERO, config(EvaluationAlgorithm::Backward));

        let lentz_function = Exp::with_config(Fixed::ZERO, config(EvaluationAlgorithm::Lentz));

        assert_eq!(
            backward_function.config().algorithm(),
            EvaluationAlgorithm::Backward,
        );

        assert_eq!(
            lentz_function.config().algorithm(),
            EvaluationAlgorithm::Lentz,
        );

        let input = q16(1);

        let backward = Function::eval(&backward_function, input).unwrap();

        let lentz = Function::eval(&lentz_function, input).unwrap();

        assert_close(backward, lentz, 128);
    }

    #[test]
    fn rejects_zero_depth_for_stored_input_evaluation() {
        let config = EvaluationConfig::binary(16, 0);

        assert_eq!(
            Evaluate::evaluate(&Exp::with_config(q16(1), config,),),
            Err(Error::InvalidDepth),
        );
    }

    #[test]
    fn rejects_zero_depth_for_function_trait_evaluation() {
        let function = Exp::with_config(Fixed::ZERO, EvaluationConfig::binary(16, 0));

        assert_eq!(Function::eval(&function, q16(1),), Err(Error::InvalidDepth),);
    }
}
