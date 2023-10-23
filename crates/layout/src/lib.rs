//! OpenType script, feature, lookup, coverage, and class-definition transformations.
//!
//! Layout retention remains independent from outline decoding and communicates through SFNT
//! primitives and explicit glyph selections.

#![forbid(unsafe_code)]

mod class_definition;
mod coverage;
mod feature;
mod feature_list;
mod gsub;
mod language_system;
mod lookup;
mod lookup_flags;
mod lookup_list;
mod mark_filter;
mod script;
mod script_list;

pub use class_definition::ClassDefinition;
pub use coverage::Coverage;
pub use feature::Feature;
pub use feature_list::FeatureList;
pub use gsub::{
    AlternateSubstitution, ChainedContextSubstitution, ContextRule, ContextSubstitution,
    GsubClosure, GsubLookup, GsubLookupList, GsubSubtable, LigatureRule, LigatureSubstitution,
    MultipleSubstitution, RemappedGsub, RetainedGsub, ReverseChainSingleSubstitution,
    SingleSubstitution, SubstitutionRecord,
};
pub use language_system::LanguageSystem;
pub use lookup::Lookup;
pub use lookup_flags::LookupFlags;
pub use lookup_list::LookupList;
pub use mark_filter::MarkFilter;
pub use script::Script;
pub use script_list::ScriptList;

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

const fn closure_error(cause: glyphweave_sfnt::LayoutCause) -> glyphweave_sfnt::Diagnostic {
    glyphweave_sfnt::Diagnostic::failure(
        glyphweave_sfnt::Stage::Closure,
        glyphweave_sfnt::Cause::Layout(cause),
    )
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn relative_target_span(
    owner: glyphweave_sfnt::ByteSpan,
    field_position: usize,
    offset: u16,
    minimum: usize,
    target: glyphweave_sfnt::LayoutTarget,
) -> glyphweave_sfnt::Decoded<glyphweave_sfnt::ByteSpan> {
    relative_target_span_u32(owner, field_position, 2, u32::from(offset), minimum, target)
}

fn relative_target_span_u32(
    owner: glyphweave_sfnt::ByteSpan,
    field_position: usize,
    field_size: usize,
    offset: u32,
    minimum: usize,
    target: glyphweave_sfnt::LayoutTarget,
) -> glyphweave_sfnt::Decoded<glyphweave_sfnt::ByteSpan> {
    let container_length = owner.length();
    let relative = usize::try_from(offset).unwrap_or(usize::MAX);
    if relative < minimum || relative >= container_length {
        return Err(layout_error_sized(
            field_position,
            field_size,
            glyphweave_sfnt::LayoutCause::InvalidLayoutOffset {
                target,
                offset,
                minimum,
                container_length,
            },
        ));
    }
    let position = owner.start() + relative;
    Ok(glyphweave_sfnt::ByteSpan::between(position, owner.end())
        .expect("validated relative target remains inside its owner"))
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
