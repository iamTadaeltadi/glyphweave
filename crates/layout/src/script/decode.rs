use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutListKind, LayoutTarget, Reader, Tag};

use super::Script;
use crate::{LanguageSystem, layout_error_sized, relative_target_span};

impl Script {
    /// Decodes a Script table at the beginning of `span`.
    /// # Errors
    /// Returns a span-aware diagnostic for truncation, malformed or unordered language Tags,
    /// duplicate language Tags, and relative targets outside the Script container.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let default_field = reader.offset();
        let default_offset = reader.u16()?;
        let count = reader.u16()?;
        let mut records = Vec::with_capacity(usize::from(count));
        let mut previous: Option<Tag> = None;

        for _ in 0..count {
            let tag_position = reader.offset();
            let tag = reader.tag()?;
            if !tag.is_well_formed() {
                return Err(layout_error_sized(
                    tag_position,
                    4,
                    LayoutCause::InvalidLayoutTag {
                        list: LayoutListKind::Language,
                        tag,
                    },
                ));
            }
            if let Some(preceding) = previous {
                let cause = if tag < preceding {
                    LayoutCause::UnsortedLayoutTags {
                        list: LayoutListKind::Language,
                        previous: preceding,
                        current: tag,
                    }
                } else if tag == preceding {
                    LayoutCause::DuplicateLayoutTag {
                        list: LayoutListKind::Language,
                        tag,
                    }
                } else {
                    previous = Some(tag);
                    let offset_position = reader.offset();
                    records.push((tag, offset_position, reader.u16()?));
                    continue;
                };
                return Err(layout_error_sized(tag_position, 4, cause));
            }
            previous = Some(tag);
            let offset_position = reader.offset();
            records.push((tag, offset_position, reader.u16()?));
        }

        let minimum = reader.offset() - span.start();
        let default_language = (default_offset != 0)
            .then(|| {
                relative_target_span(
                    span,
                    default_field,
                    default_offset,
                    minimum,
                    LayoutTarget::DefaultLanguageSystem,
                )
                .and_then(|target| LanguageSystem::decode(bytes, target))
            })
            .transpose()?;
        let mut languages = Vec::with_capacity(records.len());
        for (tag, field, offset) in records {
            let target =
                relative_target_span(span, field, offset, minimum, LayoutTarget::LanguageSystem)?;
            languages.push((tag, LanguageSystem::decode(bytes, target)?));
        }
        Ok(Self::new(default_language, languages))
    }
}
