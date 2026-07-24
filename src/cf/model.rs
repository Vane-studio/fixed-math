//! Continued-fraction evaluation model.

use crate::error::{Error, Result};

/// Describes a finite continued-fraction evaluation.
///
/// The actual coefficients are supplied by a coefficient provider.
/// This model only stores the maximum number of coefficient levels
/// that should be evaluated.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ContinuedFraction {
    depth: usize,
}

impl ContinuedFraction {
    #[inline]
    pub const fn new(depth: usize) -> Self {
        Self { depth }
    }

    #[inline]
    pub const fn depth(self) -> usize {
        self.depth
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.depth == 0
    }

    #[inline]
    pub const fn validate(self) -> Result<()> {
        if self.depth == 0 {
            Err(Error::InvalidDepth)
        } else {
            Ok(())
        }
    }

    /// Returns a new model using another evaluation depth.
    #[inline]
    pub const fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Replaces the evaluation depth in place.
    #[inline]
    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }
}

#[cfg(test)]
mod tests {
    use super::ContinuedFraction;
    use crate::error::Error;

    #[test]
    fn stores_depth() {
        let fraction = ContinuedFraction::new(8);

        assert_eq!(fraction.depth(), 8,);

        assert!(!fraction.is_empty());
    }

    #[test]
    fn builder_changes_depth() {
        let fraction = ContinuedFraction::new(4).with_depth(10);

        assert_eq!(fraction.depth(), 10,);
    }

    #[test]
    fn setter_changes_depth() {
        let mut fraction = ContinuedFraction::new(4);

        fraction.set_depth(10);

        assert_eq!(fraction.depth(), 10,);
    }

    #[test]
    fn empty_when_depth_is_zero() {
        let fraction = ContinuedFraction::default();

        assert!(fraction.is_empty());

        assert_eq!(fraction.validate(), Err(Error::InvalidDepth),);
    }

    #[test]
    fn positive_depth_is_valid() {
        let fraction = ContinuedFraction::new(1);

        assert_eq!(fraction.validate(), Ok(()),);
    }
}
