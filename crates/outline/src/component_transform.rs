//! Canonical transform forms for composite-glyph components.

use crate::F2Dot14;

/// A component's exact two-dimensional linear transform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComponentTransform {
    /// Leaves component coordinates unchanged.
    Identity,
    /// Applies one scale to both coordinate axes.
    Uniform {
        /// The common horizontal and vertical scale.
        scale: F2Dot14,
    },
    /// Applies independent horizontal and vertical scales.
    Axis {
        /// The horizontal scale.
        x_scale: F2Dot14,
        /// The vertical scale.
        y_scale: F2Dot14,
    },
    /// Applies the complete TrueType two-by-two transform.
    Matrix {
        /// Contribution from x to transformed x.
        x_scale: F2Dot14,
        /// Contribution from x to transformed y.
        scale_01: F2Dot14,
        /// Contribution from y to transformed x.
        scale_10: F2Dot14,
        /// Contribution from y to transformed y.
        y_scale: F2Dot14,
    },
}

impl ComponentTransform {
    /// Returns the identity transformation.
    #[must_use]
    pub const fn identity() -> Self {
        Self::Identity
    }

    /// Expands every compact form into x-scale, scale-01, scale-10, and y-scale coefficients.
    #[must_use]
    pub const fn coefficients(self) -> [F2Dot14; 4] {
        match self {
            Self::Identity => [F2Dot14::ONE, F2Dot14::ZERO, F2Dot14::ZERO, F2Dot14::ONE],
            Self::Uniform { scale } => [scale, F2Dot14::ZERO, F2Dot14::ZERO, scale],
            Self::Axis { x_scale, y_scale } => [x_scale, F2Dot14::ZERO, F2Dot14::ZERO, y_scale],
            Self::Matrix {
                x_scale,
                scale_01,
                scale_10,
                y_scale,
            } => [x_scale, scale_01, scale_10, y_scale],
        }
    }
}
