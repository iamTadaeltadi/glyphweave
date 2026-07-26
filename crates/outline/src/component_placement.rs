//! Canonical placement semantics for composite-glyph components.

/// Whether a transformed component offset is scaled before placement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ComponentOffsetMode {
    /// Uses the platform-compatible unscaled default behavior.
    #[default]
    Default,
    /// Applies the component transform to the offset vector.
    Scaled,
    /// Applies the offset after the component transform.
    Unscaled,
}

/// The placement arguments for one composite-glyph component.
///
/// ```
/// use glyphweave_outline::{ComponentOffsetMode, ComponentPlacement};
///
/// let placement = ComponentPlacement::Offset {
///     x: 12,
///     y: -4,
///     round_to_grid: true,
///     mode: ComponentOffsetMode::Unscaled,
/// };
/// assert!(matches!(placement, ComponentPlacement::Offset { x: 12, .. }));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComponentPlacement {
    /// Positions the component by a signed design-unit offset.
    Offset {
        /// Horizontal component offset.
        x: i16,
        /// Vertical component offset.
        y: i16,
        /// Whether the resulting offset is rounded to the pixel grid.
        round_to_grid: bool,
        /// Whether the component transform applies to this vector.
        mode: ComponentOffsetMode,
    },
    /// Aligns a child point with a point already accumulated in the parent.
    MatchPoints {
        /// Point index in the accumulated parent outline.
        parent: u16,
        /// Point index in the child component outline.
        child: u16,
    },
}

#[cfg(test)]
mod tests;
