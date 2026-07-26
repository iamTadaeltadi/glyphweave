//! Bounded parsing and eager resolution of format-4 segment arrays.

use super::CmapFormat4;
use crate::{ByteSpan, Cause, CharacterMap, Decoded, Diagnostic, GlyphId, Reader, Stage};

const FORMAT: u16 = 4;
const MINIMUM_LENGTH: usize = 24;

#[derive(Clone, Copy, Debug)]
struct Header {
    span: ByteSpan,
    language: u16,
    segment_count: u16,
    end_codes: ByteSpan,
    start_codes: ByteSpan,
    deltas: ByteSpan,
    range_offsets: ByteSpan,
    glyph_array: ByteSpan,
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    index: u16,
    start: u16,
    end: u16,
    delta: u16,
    range_offset: u16,
    range_offset_word: usize,
    span: ByteSpan,
    delta_span: ByteSpan,
    range_offset_span: ByteSpan,
}

pub(super) fn decode(bytes: &[u8], offset: usize) -> Decoded<CmapFormat4> {
    let header = decode_header(bytes, offset)?;
    let segments = decode_segments(bytes, header)?;
    let character_map = resolve_segments(bytes, header, &segments)?;
    Ok(CmapFormat4 {
        span: header.span,
        language: header.language,
        segment_count: header.segment_count,
        character_map,
    })
}

fn decode_header(bytes: &[u8], offset: usize) -> Decoded<Header> {
    let available = bytes.len().saturating_sub(offset);
    let mut reader = Reader::at(bytes, offset)?;
    let format = reader.u16()?;
    if format != FORMAT {
        return Err(failure(
            field_span(offset, 0, 2),
            Cause::UnsupportedCmapFormat { found: format },
        ));
    }

    let declared = reader.u16()?;
    if declared % 2 != 0 {
        return Err(failure(
            field_span(offset, 2, 2),
            Cause::OddCmapLength { declared },
        ));
    }
    let declared_length = usize::from(declared);
    if declared_length < MINIMUM_LENGTH || declared_length > available {
        return Err(invalid_length(offset, declared, MINIMUM_LENGTH, available));
    }
    let span = ByteSpan::sized(offset, declared_length)
        .ok_or_else(|| invalid_length(offset, declared, MINIMUM_LENGTH, available))?;

    let language = reader.u16()?;
    let encoded_segment_count = reader.u16()?;
    if encoded_segment_count == 0 || encoded_segment_count % 2 != 0 {
        return Err(failure(
            field_span(offset, 6, 2),
            Cause::InvalidCmapSegmentCount {
                encoded: encoded_segment_count,
            },
        ));
    }
    let segment_count = encoded_segment_count / 2;
    let actual_search = [reader.u16()?, reader.u16()?, reader.u16()?];
    let expected_search = derive_search_parameters(segment_count);
    if actual_search != expected_search {
        return Err(failure(
            field_span(offset, 8, 6),
            Cause::InvalidCmapSearchParameters {
                expected: expected_search,
                actual: actual_search,
            },
        ));
    }

    let array_length = usize::from(segment_count) * 2;
    let minimum_length = 16 + usize::from(segment_count) * 8;
    if declared_length < minimum_length {
        return Err(invalid_length(offset, declared, minimum_length, available));
    }
    let end_codes = field_span(offset, 14, array_length);
    let reserved_pad_offset = end_codes.end();
    let reserved_pad = read_word(bytes, reserved_pad_offset)?;
    if reserved_pad != 0 {
        return Err(failure(
            field_span(reserved_pad_offset, 0, 2),
            Cause::InvalidCmapReservedPad {
                found: reserved_pad,
            },
        ));
    }
    let start_codes = field_span(reserved_pad_offset, 2, array_length);
    let deltas = field_span(start_codes.end(), 0, array_length);
    let range_offsets = field_span(deltas.end(), 0, array_length);
    let glyph_array = ByteSpan::between(range_offsets.end(), span.end())
        .unwrap_or_else(|| ByteSpan::empty_at(span.end()));

    Ok(Header {
        span,
        language,
        segment_count,
        end_codes,
        start_codes,
        deltas,
        range_offsets,
        glyph_array,
    })
}

fn decode_segments(bytes: &[u8], header: Header) -> Decoded<Vec<Segment>> {
    let mut segments = Vec::with_capacity(usize::from(header.segment_count));
    let mut previous_end = None;
    for index in 0..header.segment_count {
        let relative = usize::from(index) * 2;
        let end_span = field_span(header.end_codes.start(), relative, 2);
        let start_span = field_span(header.start_codes.start(), relative, 2);
        let delta_span = field_span(header.deltas.start(), relative, 2);
        let range_offset_span = field_span(header.range_offsets.start(), relative, 2);
        let end = read_word(bytes, end_span.start())?;
        let start = read_word(bytes, start_span.start())?;
        if start > end || previous_end.is_some_and(|preceding| start <= preceding) {
            return Err(failure(
                start_span.join(end_span),
                Cause::InvalidCmapSegment {
                    index,
                    start,
                    end,
                    previous_end,
                },
            ));
        }
        segments.push(Segment {
            index,
            start,
            end,
            delta: read_word(bytes, delta_span.start())?,
            range_offset: read_word(bytes, range_offset_span.start())?,
            range_offset_word: range_offset_span.start(),
            span: start_span.join(end_span),
            delta_span,
            range_offset_span,
        });
        previous_end = Some(end);
    }

    let final_segment = segments
        .last()
        .copied()
        .ok_or_else(|| failure(header.span, Cause::InvalidCmapSegmentCount { encoded: 0 }))?;
    if final_segment.start != u16::MAX || final_segment.end != u16::MAX {
        return Err(failure(
            final_segment.span,
            Cause::InvalidCmapSentinelRange {
                start: final_segment.start,
                end: final_segment.end,
            },
        ));
    }
    Ok(segments)
}

fn resolve_segments(bytes: &[u8], header: Header, segments: &[Segment]) -> Decoded<CharacterMap> {
    let mut character_map = CharacterMap::new();
    for segment in segments {
        for codepoint in segment.start..=segment.end {
            let (glyph, source_span) = resolve_glyph(bytes, header, *segment, codepoint)?;
            if codepoint == u16::MAX {
                if !glyph.is_notdef() {
                    return Err(failure(
                        source_span,
                        Cause::InvalidCmapSentinel {
                            start: segment.start,
                            end: segment.end,
                            glyph,
                        },
                    ));
                }
                continue;
            }
            if glyph.is_notdef() {
                continue;
            }
            let Some(scalar) = char::from_u32(u32::from(codepoint)) else {
                return Err(failure(
                    source_span,
                    Cause::CmapSurrogateMapping { codepoint, glyph },
                ));
            };
            character_map.insert(scalar, glyph);
        }
    }
    Ok(character_map)
}

fn resolve_glyph(
    bytes: &[u8],
    header: Header,
    segment: Segment,
    codepoint: u16,
) -> Decoded<(GlyphId, ByteSpan)> {
    if segment.range_offset == 0 {
        return Ok((
            GlyphId::new(codepoint.wrapping_add(segment.delta)),
            segment.delta_span,
        ));
    }

    let target = segment
        .range_offset_word
        .checked_add(usize::from(segment.range_offset))
        .and_then(|base| {
            usize::from(codepoint - segment.start)
                .checked_mul(2)
                .and_then(|displacement| base.checked_add(displacement))
        });
    let valid_target = target.filter(|target| {
        segment.range_offset % 2 == 0
            && *target >= header.glyph_array.start()
            && target
                .checked_add(2)
                .is_some_and(|end| end <= header.glyph_array.end())
    });
    let Some(target) = valid_target else {
        return Err(failure(
            segment.range_offset_span,
            Cause::InvalidCmapRangeOffset {
                segment: segment.index,
                codepoint,
                target: target.unwrap_or(usize::MAX),
                glyph_array: header.glyph_array,
            },
        ));
    };
    let glyph_span = field_span(target, 0, 2);
    let raw_glyph = read_word(bytes, target)?;
    let resolved = if raw_glyph == 0 {
        GlyphId::NOTDEF
    } else {
        GlyphId::new(raw_glyph.wrapping_add(segment.delta))
    };
    Ok((resolved, glyph_span))
}

const fn derive_search_parameters(segment_count: u16) -> [u16; 3] {
    let mut search_segments = 1_u16;
    let mut entry_selector = 0_u16;
    while search_segments <= segment_count / 2 {
        search_segments *= 2;
        entry_selector += 1;
    }
    let search_range = search_segments * 2;
    [
        search_range,
        entry_selector,
        segment_count * 2 - search_range,
    ]
}

fn read_word(bytes: &[u8], offset: usize) -> Decoded<u16> {
    Reader::at(bytes, offset)?.u16()
}

fn invalid_length(offset: usize, declared: u16, minimum: usize, available: usize) -> Diagnostic {
    failure(
        field_span(offset, 2, 2),
        Cause::InvalidCmapLength {
            declared,
            minimum,
            available,
        },
    )
}

const fn failure(span: ByteSpan, cause: Cause) -> Diagnostic {
    Diagnostic::at(Stage::Decode, span, cause)
}

fn field_span(base: usize, relative: usize, length: usize) -> ByteSpan {
    base.checked_add(relative)
        .and_then(|start| ByteSpan::sized(start, length))
        .unwrap_or_else(|| ByteSpan::empty_at(base))
}
