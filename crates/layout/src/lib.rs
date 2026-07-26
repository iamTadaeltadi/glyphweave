//! OpenType script, feature, lookup, coverage, and class-definition transformations.
//!
//! Layout retention remains independent from outline decoding and communicates through SFNT
//! primitives and explicit glyph selections.

#![forbid(unsafe_code)]

mod class_definition;
mod coverage;
mod feature;
mod language_system;
mod script;

pub use class_definition::ClassDefinition;
pub use coverage::Coverage;
pub use feature::Feature;
pub use language_system::LanguageSystem;
pub use script::Script;

fn layout_error(offset: usize, cause: glyphweave_sfnt::LayoutCause) -> glyphweave_sfnt::Diagnostic {
    layout_error_sized(offset, 2, cause)
}

fn layout_error_sized(
    offset: usize,
    size: usize,
    cause: glyphweave_sfnt::LayoutCause,
) -> glyphweave_sfnt::Diagnostic {
    let span = glyphweave_sfnt::ByteSpan::sized(offset, size)
        .unwrap_or_else(|| glyphweave_sfnt::ByteSpan::empty_at(offset));
    glyphweave_sfnt::Diagnostic::at(
        glyphweave_sfnt::Stage::Decode,
        span,
        glyphweave_sfnt::Cause::Layout(cause),
    )
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

#[cfg(test)]
mod tests {
    use glyphweave_sfnt::{ByteSpan, LayoutCause};

    #[test]
    fn impossible_field_span_overflow_still_has_a_location() {
        let diagnostic = super::layout_error_sized(
            usize::MAX,
            4,
            LayoutCause::UnsupportedCoverageFormat { format: 0 },
        );
        assert_eq!(diagnostic.span(), Some(ByteSpan::empty_at(usize::MAX)));
    }
}
