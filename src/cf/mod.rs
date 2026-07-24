//! Continued-fraction framework.

pub mod algorithm;
pub mod atanh;
pub mod coefficient;
pub mod evaluator;
pub mod lambert;
pub mod model;

pub use algorithm::EvaluationAlgorithm;
pub use atanh::Atanh;

pub use coefficient::{Coefficient, CoefficientProvider};

pub use evaluator::Evaluator;
pub use lambert::Lambert;
pub use model::ContinuedFraction;
