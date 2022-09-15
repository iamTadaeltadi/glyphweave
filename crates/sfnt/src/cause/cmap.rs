//! Typed causes produced while decoding character-map subtables.

mod display;

/// A format-specific cause produced by a `cmap` decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CmapCause {
    /// A `cmap` subtable used a format other than the decoder expected.
    UnsupportedCmapFormat {
        /// The format required by the selected decoder.
        expected: u16,
        /// The encoded subtable format.
        found: u16,
    },
    /// A format-4 declared length cannot contain its arrays or exceeds available bytes.
    InvalidCmapLength {
        /// The encoded byte length.
        declared: u16,
        /// The minimum byte length implied by the segment count.
        minimum: usize,
        /// The bytes available from the subtable offset.
        available: usize,
    },
    /// A format-4 subtable declared an odd byte length.
    OddCmapLength {
        /// The odd encoded byte length.
        declared: u16,
    },
    /// A format-12 declared length cannot contain its header or exceeds available bytes.
    InvalidCmap12Length {
        /// The encoded byte length.
        declared: u32,
        /// The fixed minimum byte length.
        minimum: usize,
        /// The bytes available from the subtable offset.
        available: usize,
    },
    /// A format-12 declared length disagrees with its fixed-width group count.
    Cmap12LengthMismatch {
        /// The encoded byte length.
        declared: u32,
        /// The exact byte length implied by the group count.
        expected: u64,
        /// The encoded number of groups.
        groups: u32,
    },
    /// The required format-12 reserved word was nonzero.
    InvalidCmap12Reserved {
        /// The encoded reserved value.
        found: u16,
    },
    /// A format-12 group is inverted, not ordered, or overlaps its predecessor.
    InvalidCmap12Group {
        /// The zero-based group index.
        index: u32,
        /// The group's inclusive start character code.
        start: u32,
        /// The group's inclusive end character code.
        end: u32,
        /// The preceding group's end code, when this is not the first group.
        previous_end: Option<u32>,
    },
    /// A sequential format-12 mapping cannot be represented as a Glyph ID.
    Cmap12GlyphOutOfRange {
        /// The zero-based group index.
        group: u32,
        /// The character code whose mapping first exceeds the Glyph ID domain.
        codepoint: u32,
        /// The calculated glyph value.
        glyph: u64,
    },
    /// A cmap entry maps a non-scalar Unicode value to a nonzero glyph.
    CmapNonScalarMapping {
        /// The cmap subtable format.
        format: u16,
        /// The invalid Unicode value.
        codepoint: u32,
        /// The invalid nonzero Glyph ID.
        glyph: crate::GlyphId,
    },
    /// A format-4 `segCountX2` field is zero or odd.
    InvalidCmapSegmentCount {
        /// The invalid encoded value.
        encoded: u16,
    },
    /// Format-4 search fields disagree with values derived from the segment count.
    InvalidCmapSearchParameters {
        /// The derived `(searchRange, entrySelector, rangeShift)` values.
        expected: [u16; 3],
        /// The values stored in the subtable.
        actual: [u16; 3],
    },
    /// The required format-4 reserved padding word was nonzero.
    InvalidCmapReservedPad {
        /// The encoded reserved value.
        found: u16,
    },
    /// A format-4 segment is inverted, not ordered, or overlaps its predecessor.
    InvalidCmapSegment {
        /// The zero-based segment index.
        index: u16,
        /// The segment's inclusive start code.
        start: u16,
        /// The segment's inclusive end code.
        end: u16,
        /// The preceding segment's end code, when this is not the first segment.
        previous_end: Option<u16>,
    },
    /// The final format-4 segment is not the required `0xFFFF..=0xFFFF` range.
    InvalidCmapSentinelRange {
        /// The final segment start code.
        start: u16,
        /// The final segment end code.
        end: u16,
    },
    /// The required final format-4 sentinel maps `0xFFFF` to a nonzero glyph.
    InvalidCmapSentinel {
        /// The final segment start code.
        start: u16,
        /// The final segment end code.
        end: u16,
        /// The resolved sentinel Glyph ID.
        glyph: crate::GlyphId,
    },
    /// A format-4 range offset targets a word outside `glyphIdArray`.
    InvalidCmapRangeOffset {
        /// The zero-based segment index.
        segment: u16,
        /// The character code being resolved.
        codepoint: u16,
        /// The calculated absolute byte offset.
        target: usize,
        /// The legal glyph-array range.
        glyph_array: crate::ByteSpan,
    },
    /// A format-4 entry maps a UTF-16 surrogate to a nonzero glyph.
    CmapSurrogateMapping {
        /// The surrogate code unit.
        codepoint: u16,
        /// The invalid nonzero Glyph ID.
        glyph: crate::GlyphId,
    },
}
