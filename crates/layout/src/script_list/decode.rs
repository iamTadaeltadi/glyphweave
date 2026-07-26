use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutListKind, LayoutTarget, Reader, Tag};

use super::ScriptList;
use crate::{Script, layout_error_sized, relative_target_span};

impl ScriptList {
    /// Decodes a Script List at the beginning of `span`.
    /// # Errors
    /// Returns a diagnostic for truncation, invalid ordering or Tags, duplicates, or bad targets.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
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
                        list: LayoutListKind::Script,
                        tag,
                    },
                ));
            }
            if let Some(preceding) = previous {
                let cause = if tag < preceding {
                    Some(LayoutCause::UnsortedLayoutTags {
                        list: LayoutListKind::Script,
                        previous: preceding,
                        current: tag,
                    })
                } else if tag == preceding {
                    Some(LayoutCause::DuplicateLayoutTag {
                        list: LayoutListKind::Script,
                        tag,
                    })
                } else {
                    None
                };
                if let Some(cause) = cause {
                    return Err(layout_error_sized(tag_position, 4, cause));
                }
            }
            previous = Some(tag);
            let offset_position = reader.offset();
            records.push((tag, offset_position, reader.u16()?));
        }

        let minimum = reader.offset() - span.start();
        let mut scripts = Vec::with_capacity(records.len());
        for (tag, field, offset) in records {
            let target = relative_target_span(span, field, offset, minimum, LayoutTarget::Script)?;
            scripts.push((tag, Script::decode(bytes, target)?));
        }
        Ok(Self::new(scripts))
    }
}
