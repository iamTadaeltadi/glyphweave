//! Canonical locations of TrueType glyph descriptions.

mod decode;

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LocaFormat};

/// Validated `loca` offsets paired with the `glyf` table they delimit.
///
/// The canonical offsets are byte counts relative to [`Loca::glyf_span`], regardless of whether
/// the source used short half-offsets or long byte offsets. There is one more offset than the
/// reported Glyph count so every in-range Glyph ID has a complete half-open Byte Span. Consecutive
/// equal offsets preserve a glyph without outline data as an empty span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loca {
    pub(crate) table_span: ByteSpan,
    pub(crate) glyf_span: ByteSpan,
    pub(crate) format: LocaFormat,
    pub(crate) glyph_count: u16,
    pub(crate) offsets: Vec<u32>,
}

impl Loca {
    /// Decodes a `loca` table and resolves its offsets against `glyf`.
    ///
    /// ```
    /// use glyphweave_outline::Loca;
    /// use glyphweave_sfnt::{ByteSpan, LocaFormat};
    ///
    /// let bytes = [0, 0];
    /// let loca = ByteSpan::sized(0, 2).unwrap();
    /// let glyf = ByteSpan::empty_at(2);
    /// let decoded = Loca::decode(&bytes, loca, glyf, 0, LocaFormat::Short)?;
    /// assert_eq!(decoded.offsets(), [0]);
    /// # Ok::<(), glyphweave_sfnt::Diagnostic>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic when either table span is out of bounds or the location
    /// array's length and offsets violate the selected format, Glyph count, or `glyf` bounds.
    pub fn decode(
        bytes: &[u8],
        table_span: ByteSpan,
        glyf_span: ByteSpan,
        glyph_count: u16,
        format: LocaFormat,
    ) -> Decoded<Self> {
        decode::decode(bytes, table_span, glyf_span, glyph_count, format)
    }

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

    /// Returns all canonical byte offsets relative to `glyf`, including the final sentinel.
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
