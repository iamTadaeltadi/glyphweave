//! Human-readable rendering for character-map failures.

use core::fmt;

use super::CmapCause;

impl CmapCause {
    fn fmt_four(self, f: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        let rendered = match self {
            Self::InvalidCmapLength {
                declared,
                minimum,
                available,
            } => write!(
                f,
                "cmap format 4 length {declared} needs at least {minimum} byte(s) with {available} available"
            ),
            Self::OddCmapLength { declared } => {
                write!(f, "cmap format 4 length {declared} is not even")
            }
            Self::InvalidCmapSegmentCount { encoded } => {
                write!(
                    f,
                    "cmap format 4 segCountX2 {encoded} is not positive and even"
                )
            }
            Self::InvalidCmapSearchParameters { expected, actual } => write!(
                f,
                "cmap format 4 search parameters {actual:?} do not match derived values {expected:?}"
            ),
            Self::InvalidCmapReservedPad { found } => {
                write!(f, "cmap format 4 reservedPad must be zero but is {found}")
            }
            Self::InvalidCmapSegment {
                index,
                start,
                end,
                previous_end,
            } => write!(
                f,
                "cmap format 4 segment {index} range {start:#06x}..={end:#06x} is invalid after {previous_end:?}"
            ),
            Self::InvalidCmapSentinelRange { start, end } => write!(
                f,
                "cmap format 4 final segment {start:#06x}..={end:#06x} is not the 0xffff sentinel"
            ),
            Self::InvalidCmapSentinel { start, end, glyph } => write!(
                f,
                "cmap format 4 sentinel {start:#06x}..={end:#06x} resolves to glyph {glyph}"
            ),
            Self::InvalidCmapRangeOffset {
                segment,
                codepoint,
                target,
                glyph_array,
            } => write!(
                f,
                "cmap format 4 segment {segment} code {codepoint:#06x} targets byte {target} outside glyph array {glyph_array}"
            ),
            Self::CmapSurrogateMapping { codepoint, glyph } => write!(
                f,
                "cmap format 4 surrogate {codepoint:#06x} resolves to glyph {glyph}"
            ),
            _ => return None,
        };
        Some(rendered)
    }

    fn fmt_twelve(self, f: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        let rendered = match self {
            Self::InvalidCmap12Length {
                declared,
                minimum,
                available,
            } => write!(
                f,
                "cmap format 12 length {declared} needs at least {minimum} byte(s) with {available} available"
            ),
            Self::Cmap12LengthMismatch {
                declared,
                expected,
                groups,
            } => write!(
                f,
                "cmap format 12 length {declared} does not equal {expected} bytes required by {groups} group(s)"
            ),
            Self::InvalidCmap12Reserved { found } => {
                write!(
                    f,
                    "cmap format 12 reserved word must be zero but is {found}"
                )
            }
            Self::InvalidCmap12Group {
                index,
                start,
                end,
                previous_end,
            } => write!(
                f,
                "cmap format 12 group {index} range {start:#010x}..={end:#010x} is invalid after {previous_end:?}"
            ),
            Self::Cmap12GlyphOutOfRange {
                group,
                codepoint,
                glyph,
            } => write!(
                f,
                "cmap format 12 group {group} code {codepoint:#010x} resolves to glyph {glyph} outside the 16-bit range"
            ),
            Self::CmapNonScalarMapping {
                format,
                codepoint,
                glyph,
            } => write!(
                f,
                "cmap format {format} code {codepoint:#010x} resolves to glyph {glyph} but is not a Unicode scalar"
            ),
            _ => return None,
        };
        Some(rendered)
    }
}

impl fmt::Display for CmapCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnsupportedCmapFormat { expected, found } => write!(
                f,
                "expected cmap format {expected} but found format {found}"
            ),
            Self::InvalidCmapLength { .. }
            | Self::OddCmapLength { .. }
            | Self::InvalidCmapSegmentCount { .. }
            | Self::InvalidCmapSearchParameters { .. }
            | Self::InvalidCmapReservedPad { .. }
            | Self::InvalidCmapSegment { .. }
            | Self::InvalidCmapSentinelRange { .. }
            | Self::InvalidCmapSentinel { .. }
            | Self::InvalidCmapRangeOffset { .. }
            | Self::CmapSurrogateMapping { .. } => self
                .fmt_four(f)
                .unwrap_or_else(|| f.write_str("unrendered cmap format 4 failure")),
            Self::InvalidCmap12Length { .. }
            | Self::Cmap12LengthMismatch { .. }
            | Self::InvalidCmap12Reserved { .. }
            | Self::InvalidCmap12Group { .. }
            | Self::Cmap12GlyphOutOfRange { .. }
            | Self::CmapNonScalarMapping { .. } => self
                .fmt_twelve(f)
                .unwrap_or_else(|| f.write_str("unrendered cmap format 12 failure")),
        }
    }
}
