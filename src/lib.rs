//! Fixed-point mathematical function prototype.
//!
//! The crate currently provides:
//!
//! - a signed prototype numeric type;
//! - generalized continued-fraction infrastructure;
//! - Lambert continued-fraction coefficients;
//! - configurable exponential and logarithm evaluators.
//!
//! The implementation is intentionally incremental. Public module
//! boundaries are kept stable while the underlying arithmetic evolves.

#![forbid(unsafe_code)]

pub mod cf;
pub mod error;
pub mod function;
pub mod number;
pub mod traits;

pub mod prelude;

pub use error::{Error, Result};
pub use number::fixed::Fixed;
