//! Decoding of one complete component record.

use glyphweave_sfnt::{Decoded, GlyphId, Reader};

use super::flags::ComponentFlags;
use crate::Component;

pub(super) struct DecodedComponent {
    pub(super) component: Component,
    pub(super) flags: ComponentFlags,
}

pub(super) fn decode(reader: &mut Reader<'_>, component_index: u32) -> Decoded<DecodedComponent> {
    let flags = ComponentFlags::decode(reader, component_index)?;
    let glyph_id = GlyphId::new(reader.u16()?);
    let placement = super::arguments::decode(reader, flags)?;
    let transform = super::transform::decode(reader, flags)?;
    Ok(DecodedComponent {
        component: Component::new(glyph_id, placement, transform, flags.uses_child_metrics()),
        flags,
    })
}
