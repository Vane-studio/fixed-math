//! Numeric types and fixed-point configuration.

pub mod constant;
pub mod fixed;
pub mod format;
pub mod rounding;

pub use fixed::Fixed;
pub use format::Format;
pub use rounding::RoundingMode;
