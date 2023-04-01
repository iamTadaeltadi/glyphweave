//! Eagerly decoded TrueType outline collection.

mod decode;

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId};

use crate::{Loca, Outline};

/// Canonical outlines indexed by input Glyph ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Glyf {
    pub(crate) table_span: ByteSpan,
    pub(crate) outlines: Vec<Outline>,
}

impl Glyf {
    /// Visits every canonical outline paired with its input Glyph ID.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (GlyphId, &Outline)> + '_ {
        self.outlines.iter().enumerate().map(|(index, outline)| {
            let raw = u16::try_from(index).unwrap_or(u16::MAX);
            (GlyphId::new(raw), outline)
        })
    }

    /// Decodes every Glyph Byte Span delimited by a validated `loca` table.
    ///
    /// Empty intervals remain explicit, while non-empty descriptions are classified and fully
    /// decoded as simple or composite outlines.
    ///
    /// ```
    /// use glyphweave_outline::{Glyf, Loca};
    /// use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};
    ///
    /// let bytes = [0; 4];
    /// let loca = Loca::decode(
    ///     &bytes,
    ///     ByteSpan::sized(0, 4).unwrap(),
    ///     ByteSpan::empty_at(4),
    ///     1,
    ///     LocaFormat::Short,
    /// )?;
    /// let glyf = Glyf::decode(&bytes, &loca)?;
    /// assert!(glyf.outline(GlyphId::NOTDEF).unwrap().is_empty());
    /// # Ok::<(), glyphweave_sfnt::Diagnostic>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the first span-aware Diagnostic produced by an out-of-bounds table or malformed
    /// glyph description, in ascending input Glyph ID order.
    pub fn decode(bytes: &[u8], loca: &Loca) -> Decoded<Self> {
        decode::decode(bytes, loca)
    }

    /// Returns canonical outlines in ascending input Glyph ID order.
    #[must_use]
    pub fn outlines(&self) -> &[Outline] {
        &self.outlines
    }

    /// Borrows the canonical outline for an in-range input Glyph ID.
    #[must_use]
    pub fn outline(&self, glyph: GlyphId) -> Option<&Outline> {
        self.outlines.get(usize::from(glyph.raw()))
    }

    /// Returns the number of input Glyph IDs represented by the collection.
    #[must_use]
    pub fn glyph_count(&self) -> usize {
        self.outlines.len()
    }

    /// Returns the checked span occupied by the source `glyf` table.
    #[must_use]
    pub const fn table_span(&self) -> ByteSpan {
        self.table_span
    }
}
