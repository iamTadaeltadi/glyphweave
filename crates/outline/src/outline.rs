//! Canonical decoded forms for one input Glyph ID.

use glyphweave_sfnt::ByteSpan;

use crate::{CompositeGlyph, SimpleGlyph};

/// One decoded TrueType outline or an explicitly empty `loca` interval.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outline {
    /// A Glyph ID whose consecutive `loca` offsets are equal.
    Empty {
        /// The checked zero-length location inside `glyf`.
        span: ByteSpan,
    },
    /// A directly encoded contour outline.
    Simple(SimpleGlyph),
    /// An ordered sequence of referenced child outlines.
    Composite(CompositeGlyph),
}
