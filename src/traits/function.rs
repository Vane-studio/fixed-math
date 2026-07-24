//! Common interface for mathematical functions.

use crate::error::Result;
use crate::function::EvaluationConfig;
use crate::number::fixed::Fixed;

/// Interface implemented by mathematical functions.
pub trait Function {
    /// Returns the configuration used by this function.
    fn config(&self) -> EvaluationConfig;

    /// Evaluates the function.
    fn evaluate(&self, x: Fixed) -> Result<Fixed>;

    /// Convenience alias.
    #[inline]
    fn eval(&self, x: Fixed) -> Result<Fixed> {
        self.evaluate(x)
    }
}

#[cfg(test)]
mod tests {
    use super::Function;
    use crate::error::Result;
    use crate::function::EvaluationConfig;
    use crate::number::fixed::Fixed;

    struct Identity {
        config: EvaluationConfig,
    }

    impl Identity {
        fn new() -> Self {
            Self {
                config: EvaluationConfig::default(),
            }
        }
    }

    impl Function for Identity {
        fn config(&self) -> EvaluationConfig {
            self.config
        }

        fn evaluate(&self, x: Fixed) -> Result<Fixed> {
            Ok(x)
        }
    }

    #[test]
    fn eval_forwards_to_evaluate() {
        let function = Identity::new();

        assert_eq!(function.eval(Fixed::from_raw(15)), Ok(Fixed::from_raw(15)),);
    }

    #[test]
    fn exposes_configuration() {
        let function = Identity::new();

        assert_eq!(function.config(), EvaluationConfig::default(),);
    }
}
