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
    Ok(Loca {
        table_span,
        glyf_span,
        format,
        glyph_count,
        offsets,
    })
}
