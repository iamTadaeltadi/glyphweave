//! Decoding for one Coverage-indexed `LigatureSet`.

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LayoutCause, LayoutTarget, Reader};

use crate::{LigatureRule, layout_error, relative_target_span};

pub(super) fn decode(bytes: &[u8], span: ByteSpan, first: GlyphId) -> Decoded<Vec<LigatureRule>> {
    let mut reader = Reader::within(bytes, span)?;
    let count = reader.u16()?;
    let mut offsets = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let position = reader.offset();
        offsets.push((position, reader.u16()?));
    }
    let minimum = reader.offset() - span.start();
    let mut rules = Vec::with_capacity(offsets.len());
    for (position, offset) in offsets {
        let target = relative_target_span(span, position, offset, minimum, LayoutTarget::Ligature)?;
        rules.push(decode_ligature(bytes, target, first)?);
    }
    Ok(rules)
}

fn decode_ligature(bytes: &[u8], span: ByteSpan, first: GlyphId) -> Decoded<LigatureRule> {
    let mut reader = Reader::within(bytes, span)?;
    let replacement = GlyphId::new(reader.u16()?);
    let count_position = reader.offset();
    let count = reader.u16()?;
    if count < 2 {
        return Err(layout_error(
            count_position,
            LayoutCause::InvalidLigatureComponentCount { count },
        ));
    }
    let second = GlyphId::new(reader.u16()?);
    let mut remaining = Vec::with_capacity(usize::from(count - 2));
    for _ in 2..count {
        remaining.push(GlyphId::new(reader.u16()?));
    }
    Ok(LigatureRule::new(first, second, remaining, replacement))
}
