//! Bounded parsing and eager resolution of format-12 sequential groups.

use super::CmapFormat12;
use crate::{
    ByteSpan, Cause, CharacterMap, CmapCause, Decoded, Diagnostic, GlyphId, Reader, Stage,
};

const FORMAT: u16 = 12;
const HEADER_LENGTH: usize = 16;
const HEADER_LENGTH_U64: u64 = 16;
const GROUP_LENGTH: u64 = 12;
const FIRST_INVALID_GLYPH: u64 = 65_536;

#[derive(Clone, Copy, Debug)]
struct SequentialGlyphs {
    start: u16,
    end: u16,
}

pub(super) fn decode(bytes: &[u8], offset: usize) -> Decoded<CmapFormat12> {
    let available = bytes.len().saturating_sub(offset);
    let mut reader = Reader::at(bytes, offset)?;
    let format = reader.u16()?;
    if format != FORMAT {
        return Err(failure(
            field_span(offset, 0, 2),
            CmapCause::UnsupportedCmapFormat {
                expected: FORMAT,
                found: format,
            },
        ));
    }
    let reserved = reader.u16()?;
    if reserved != 0 {
        return Err(failure(
            field_span(offset, 2, 2),
            CmapCause::InvalidCmap12Reserved { found: reserved },
        ));
    }

    let declared = reader.u32()?;
    let declared_length = usize::try_from(declared).ok();
    if declared_length.is_none_or(|length| length < HEADER_LENGTH || length > available) {
        return Err(failure(
            field_span(offset, 4, 4),
            CmapCause::InvalidCmap12Length {
                declared,
                minimum: HEADER_LENGTH,
                available,
            },
        ));
    }
    let declared_length = declared_length.unwrap_or(HEADER_LENGTH);
    let span =
        ByteSpan::sized(offset, declared_length).unwrap_or_else(|| ByteSpan::empty_at(offset));
    let language = reader.u32()?;
    let group_count = reader.u32()?;
    let expected_length = u64::from(group_count) * GROUP_LENGTH + HEADER_LENGTH_U64;
    if u64::from(declared) != expected_length {
        return Err(failure(
            field_span(offset, 4, 4),
            CmapCause::Cmap12LengthMismatch {
                declared,
                expected: expected_length,
                groups: group_count,
            },
        ));
    }

    let character_map = decode_groups(&mut reader, group_count)?;
    Ok(CmapFormat12 {
        span,
        language,
        group_count,
        character_map,
    })
}

fn decode_groups(reader: &mut Reader<'_>, group_count: u32) -> Decoded<CharacterMap> {
    let mut character_map = CharacterMap::new();
    let mut previous_end = None;
    for index in 0..group_count {
        let group_span = field_span(reader.offset(), 0, 12);
        let start = reader.u32()?;
        let end = reader.u32()?;
        let start_glyph = reader.u32()?;
        if start > end || previous_end.is_some_and(|preceding| start <= preceding) {
            return Err(failure(
                group_span,
                CmapCause::InvalidCmap12Group {
                    index,
                    start,
                    end,
                    previous_end,
                },
            ));
        }
        let glyphs = validate_glyph_range(index, start, end, start_glyph, group_span)?;
        resolve_group(&mut character_map, start, end, glyphs, group_span)?;
        previous_end = Some(end);
    }
    Ok(character_map)
}

fn validate_glyph_range(
    group: u32,
    start: u32,
    end: u32,
    start_glyph: u32,
    span: ByteSpan,
) -> Decoded<SequentialGlyphs> {
    let final_glyph = u64::from(start_glyph) + u64::from(end - start);
    if let (Ok(start), Ok(end)) = (u16::try_from(start_glyph), u16::try_from(final_glyph)) {
        return Ok(SequentialGlyphs { start, end });
    }
    let first_invalid_offset = FIRST_INVALID_GLYPH.saturating_sub(u64::from(start_glyph));
    let codepoint = u64::from(start)
        .saturating_add(first_invalid_offset)
        .min(u64::from(end));
    Err(failure(
        span,
        CmapCause::Cmap12GlyphOutOfRange {
            group,
            codepoint: u32::try_from(codepoint).unwrap_or(end),
            glyph: u64::from(start_glyph) + (codepoint - u64::from(start)),
        },
    ))
}

fn resolve_group(
    character_map: &mut CharacterMap,
    start: u32,
    end: u32,
    glyphs: SequentialGlyphs,
    span: ByteSpan,
) -> Decoded<()> {
    for (codepoint, raw_glyph) in (start..=end).zip(glyphs.start..=glyphs.end) {
        let glyph = GlyphId::new(raw_glyph);
        if glyph.is_notdef() {
            continue;
        }
        let Some(scalar) = char::from_u32(codepoint) else {
            return Err(failure(
                span,
                CmapCause::CmapNonScalarMapping {
                    format: FORMAT,
                    codepoint,
                    glyph,
                },
            ));
        };
        character_map.insert(scalar, glyph);
    }
    Ok(())
}

const fn failure(span: ByteSpan, cause: CmapCause) -> Diagnostic {
    Diagnostic::at(Stage::Decode, span, Cause::Cmap(cause))
}

fn field_span(base: usize, relative: usize, length: usize) -> ByteSpan {
    base.checked_add(relative)
        .and_then(|start| ByteSpan::sized(start, length))
        .unwrap_or_else(|| ByteSpan::empty_at(base))
}
