//! Core error model.

use core::fmt;

/// Result type used throughout the crate.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors produced by numeric operations and function evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    /// An arithmetic operation exceeded the supported integer range.
    Overflow,

    /// An arithmetic operation produced a value below the supported range.
    Underflow,

    /// Division by zero was requested.
    DivisionByZero,

    /// The input is outside the mathematical domain of a function.
    Domain,

    /// The requested fixed-point format is invalid.
    InvalidFormat,

    /// The requested evaluation depth is invalid.
    InvalidDepth,

    /// An iterative evaluation did not converge.
    NoConvergence,

    /// The requested operation has not been implemented yet.
    NotImplemented,
}

impl Error {
    #[inline]
    pub const fn message(self) -> &'static str {
        match self {
            Self::Overflow => "numeric overflow",
            Self::Underflow => "numeric underflow",
            Self::DivisionByZero => "division by zero",
            Self::Domain => "input outside function domain",
            Self::InvalidFormat => "invalid fixed-point format",
            Self::InvalidDepth => "invalid evaluation depth",
            Self::NoConvergence => "evaluation did not converge",
            Self::NotImplemented => "operation not implemented",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message())
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn exposes_stable_error_messages() {
        assert_eq!(Error::Overflow.message(), "numeric overflow",);

        assert_eq!(Error::DivisionByZero.message(), "division by zero",);

        assert_eq!(Error::Domain.message(), "input outside function domain",);
    }

    #[test]
    fn implements_display() {
        assert_eq!(
            Error::NoConvergence.to_string(),
            "evaluation did not converge",
        );
    }
}
