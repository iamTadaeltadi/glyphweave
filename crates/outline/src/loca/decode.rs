//! Bounded expansion of short and long `loca` entries.

use glyphweave_sfnt::{
    ByteSpan, Cause, Decoded, Diagnostic, LocaFormat, OutlineCause, Reader, Stage,
};

use super::Loca;

const SHORT_WIDTH: usize = 2;
const LONG_WIDTH: usize = 4;

pub(super) fn decode(
    bytes: &[u8],
    table_span: ByteSpan,
    glyf_span: ByteSpan,
    glyph_count: u16,
    format: LocaFormat,
) -> Decoded<Loca> {
    table_span.slice(bytes)?;
    glyf_span.slice(bytes)?;

    let entry_count = usize::from(glyph_count) + 1;
    let width = match format {
        LocaFormat::Short => SHORT_WIDTH,
        LocaFormat::Long => LONG_WIDTH,
    };
    let expected = entry_count * width;
    if table_span.length() != expected {
        return Err(Diagnostic::at(
            Stage::Decode,
            table_span,
            Cause::Outline(OutlineCause::InvalidLocaLength {
                format,
                glyph_count,
                expected,
                actual: table_span.length(),
            }),
        ));
    }

    let mut reader = Reader::at(bytes, table_span.start())?;
    let mut offsets = Vec::with_capacity(entry_count);
    for _ in 0..entry_count {
        let offset = match format {
            LocaFormat::Short => u32::from(reader.u16()?) * 2,
            LocaFormat::Long => reader.u32()?,
        };
        offsets.push(offset);
    }
    if let Some(&offset) = offsets.first().filter(|&&offset| offset != 0) {
        return Err(Diagnostic::at(
            Stage::Decode,
            entry_span(table_span, 0, width),
            Cause::Outline(OutlineCause::NonzeroLocaStart { offset }),
        ));
    }
    for (preceding_index, pair) in offsets.windows(2).enumerate() {
        let index = preceding_index + 1;
        if pair[1] % 2 != 0 {
            return Err(Diagnostic::at(
                Stage::Decode,
                entry_span(table_span, index, width),
                Cause::Outline(OutlineCause::MisalignedLocaOffset {
                    index: u32::try_from(index).unwrap_or(u32::MAX),
                    offset: pair[1],
                }),
            ));
        }
        if usize::try_from(pair[1]).map_or(true, |offset| offset > glyf_span.length()) {
            return Err(Diagnostic::at(
                Stage::Decode,
                entry_span(table_span, index, width),
                Cause::Outline(OutlineCause::LocaOffsetOutOfBounds {
                    index: u32::try_from(index).unwrap_or(u32::MAX),
                    offset: pair[1],
                    glyf_length: glyf_span.length(),
                }),
            ));
        }
        if pair[1] < pair[0] {
            return Err(Diagnostic::at(
                Stage::Decode,
                entry_span(table_span, index, width),
                Cause::Outline(OutlineCause::DecreasingLocaOffset {
                    index: u32::try_from(index).unwrap_or(u32::MAX),
                    offset: pair[1],
                    previous: pair[0],
                }),
            ));
        }
    }
    let sentinel_index = entry_count - 1;
    let sentinel = offsets.last().copied().unwrap_or(0);
    if usize::try_from(sentinel).ok() != Some(glyf_span.length()) {
        return Err(Diagnostic::at(
            Stage::Decode,
            entry_span(table_span, sentinel_index, width),
            Cause::Outline(OutlineCause::LocaEndMismatch {
                index: u32::try_from(sentinel_index).unwrap_or(u32::MAX),
                offset: sentinel,
                glyf_length: glyf_span.length(),
            }),
        ));
    }
    Ok(Loca {
        table_span,
        glyf_span,
        format,
        glyph_count,
        offsets,
    })
}

fn entry_span(table: ByteSpan, index: usize, width: usize) -> ByteSpan {
    index
        .checked_mul(width)
        .and_then(|relative| table.start().checked_add(relative))
        .and_then(|start| ByteSpan::sized(start, width))
        .unwrap_or_else(|| ByteSpan::empty_at(table.start()))
}
