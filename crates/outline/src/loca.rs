//! Canonical locations of TrueType glyph descriptions.

use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};

/// Validated `loca` offsets paired with the `glyf` table they delimit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loca {
    pub(crate) table_span: ByteSpan,
    pub(crate) glyf_span: ByteSpan,
    pub(crate) format: LocaFormat,
    pub(crate) glyph_count: u16,
    pub(crate) offsets: Vec<u32>,
}

impl Loca {
    /// Returns the checked span occupied by the `loca` table.
    #[must_use]
    pub const fn table_span(&self) -> ByteSpan {
        self.table_span
    }

    /// Returns the checked `glyf` span that all offsets are relative to.
    #[must_use]
    pub const fn glyf_span(&self) -> ByteSpan {
        self.glyf_span
    }

    /// Returns the short or long format used by the source table.
    #[must_use]
    pub const fn format(&self) -> LocaFormat {
        self.format
    }

    /// Returns the number of Glyph IDs delimited by this table.
    #[must_use]
    pub const fn glyph_count(&self) -> u16 {
        self.glyph_count
    }

    /// Returns all relative offsets, including the final sentinel.
    #[must_use]
    pub fn offsets(&self) -> &[u32] {
        &self.offsets
    }

    /// Returns the absolute `glyf` Byte Span for `glyph` when it is in range.
    #[must_use]
    pub fn glyph_span(&self, glyph: GlyphId) -> Option<ByteSpan> {
        let index = usize::from(glyph.raw());
        if index >= usize::from(self.glyph_count) {
            return None;
        }
        let start = usize::try_from(*self.offsets.get(index)?).ok()?;
        let end = usize::try_from(*self.offsets.get(index + 1)?).ok()?;
        let absolute_start = self.glyf_span.start().checked_add(start)?;
        let absolute_end = self.glyf_span.start().checked_add(end)?;
        ByteSpan::between(absolute_start, absolute_end)
    }
}

#[cfg(test)]
mod tests;
