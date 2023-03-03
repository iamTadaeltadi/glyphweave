//! Canonical decoded forms for one input Glyph ID.

use glyphweave_sfnt::ByteSpan;

use crate::{CompositeGlyph, GlyphBounds, SimpleGlyph};

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

impl Outline {
    /// Returns recorded bounds for encoded outlines, or `None` for an empty interval.
    #[must_use]
    pub const fn bounds(&self) -> Option<GlyphBounds> {
        match self {
            Self::Empty { .. } => None,
            Self::Simple(glyph) => Some(glyph.bounds()),
            Self::Composite(glyph) => Some(glyph.bounds()),
        }
    }

    /// Borrows the decoded composite glyph when this outline references children.
    #[must_use]
    pub const fn as_composite(&self) -> Option<&CompositeGlyph> {
        match self {
            Self::Composite(glyph) => Some(glyph),
            Self::Empty { .. } | Self::Simple(_) => None,
        }
    }

    /// Borrows the decoded simple glyph when this outline is direct.
    #[must_use]
    pub const fn as_simple(&self) -> Option<&SimpleGlyph> {
        match self {
            Self::Simple(glyph) => Some(glyph),
            Self::Empty { .. } | Self::Composite(_) => None,
        }
    }

    /// Returns whether this Glyph ID has no encoded outline data.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }

    /// Returns the checked Glyph Byte Span represented by this outline.
    #[must_use]
    pub const fn span(&self) -> ByteSpan {
        match self {
            Self::Empty { span } => *span,
            Self::Simple(glyph) => glyph.span(),
            Self::Composite(glyph) => glyph.span(),
        }
    }
}

#[cfg(test)]
mod tests;
