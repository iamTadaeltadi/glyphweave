use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutListKind, LayoutTarget, Reader, Tag};

use super::FeatureList;
use crate::{Feature, layout_error_sized, relative_target_span};

impl FeatureList {
    /// Decodes a Feature List at the beginning of `span`.
    /// # Errors
    /// Returns a diagnostic for truncation, malformed or decreasing Tags, or invalid targets.
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
                        list: LayoutListKind::Feature,
                        tag,
                    },
                ));
            }
            if let Some(preceding) = previous {
                if tag < preceding {
                    return Err(layout_error_sized(
                        tag_position,
                        4,
                        LayoutCause::UnsortedLayoutTags {
                            list: LayoutListKind::Feature,
                            previous: preceding,
                            current: tag,
                        },
                    ));
                }
            }
            previous = Some(tag);
            let offset_position = reader.offset();
            records.push((tag, offset_position, reader.u16()?));
        }

        let minimum = reader.offset() - span.start();
        let mut features = Vec::with_capacity(records.len());
        for (tag, field, offset) in records {
            let target = relative_target_span(span, field, offset, minimum, LayoutTarget::Feature)?;
            features.push((tag, Feature::decode(bytes, target)?));
        }
        Ok(Self::new(features))
    }
}
