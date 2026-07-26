//! OpenType script, feature, lookup, coverage, and class-definition transformations.
//!
//! Layout retention remains independent from outline decoding and communicates through SFNT
//! primitives and explicit glyph selections.

#![forbid(unsafe_code)]

mod class_definition;
mod coverage;

pub use class_definition::ClassDefinition;
pub use coverage::Coverage;

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
