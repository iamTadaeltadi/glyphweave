//! GSUB semantic diagnostic rendering.

use core::fmt;

use super::super::LayoutCause;

pub(super) fn write(
    cause: &LayoutCause,
    formatter: &mut fmt::Formatter<'_>,
) -> Option<fmt::Result> {
    let result = match *cause {
        LayoutCause::UnsupportedGsubFormat {
            lookup_type,
            format,
        } => write!(
            formatter,
            "unsupported GSUB lookup type {lookup_type} format {format}"
        ),
        LayoutCause::MismatchedGsubMappingCount {
            lookup_type,
            inputs,
            records,
        } => write!(
            formatter,
            "GSUB lookup type {lookup_type} has {inputs} input glyph(s) but {records} mapping record(s)"
        ),
        LayoutCause::InvalidLigatureComponentCount { count } => write!(
            formatter,
            "GSUB Ligature declares {count} component(s), fewer than the required two"
        ),
        LayoutCause::EmptyContextInput { lookup_type } => write!(
            formatter,
            "GSUB lookup type {lookup_type} context has no input glyphs"
        ),
        LayoutCause::InvalidSubstitutionSequenceIndex {
            lookup_type,
            sequence_index,
            input_count,
        } => write!(
            formatter,
            "GSUB lookup type {lookup_type} sequence index {sequence_index} is outside {input_count} input glyph(s)"
        ),
        LayoutCause::InvalidContextLookupIndex {
            lookup_index,
            lookup_count,
        } => write!(
            formatter,
            "context references LookupList index {lookup_index} but only {lookup_count} Lookup(s) exist"
        ),
        _ => return None,
    };
    Some(result)
}
