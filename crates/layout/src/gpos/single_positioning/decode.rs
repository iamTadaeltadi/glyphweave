//! Bounded decoding for both `SinglePos` formats.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::SinglePositioning;
use crate::{Coverage, ValueFormat, ValueRecord, layout_error, relative_target_span};

impl SinglePositioning {
    /// Decodes a `SinglePos` subtable at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// For truncation, an unknown format, a reserved value format, an invalid Coverage offset,
    /// or a format 2 value count that disagrees with its Coverage.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        match reader.u16()? {
            1 => Self::decode_shared(bytes, span, &mut reader),
            2 => Self::decode_per_glyph(bytes, span, &mut reader),
            format => Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposSingleFormat { format },
            )),
        }
    }

    fn decode_shared(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let offset_position = reader.offset();
        let offset = reader.u16()?;
        let value_format = decode_value_format(reader)?;
        let record = decode_value_record(reader, value_format)?;
        let coverage = decode_coverage(bytes, span, offset_position, offset, reader.offset())?;
        Ok(Self::new(
            value_format,
            coverage
                .glyphs()
                .iter()
                .copied()
                .map(|glyph| (glyph, record)),
        ))
    }

    fn decode_per_glyph(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let offset_position = reader.offset();
        let offset = reader.u16()?;
        let value_format = decode_value_format(reader)?;
        let count_position = reader.offset();
        let count = reader.u16()?;
        let mut records = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            records.push(decode_value_record(reader, value_format)?);
        }
        let coverage = decode_coverage(bytes, span, offset_position, offset, reader.offset())?;
        if coverage.len() != records.len() {
            return Err(layout_error(
                count_position,
                LayoutCause::MismatchedGposSingleValueCount {
                    inputs: coverage.len(),
                    records: records.len(),
                },
            ));
        }
        Ok(Self::new(
            value_format,
            coverage.glyphs().iter().copied().zip(records),
        ))
    }
}

/// Reads a value format, rejecting any format that sets a reserved bit.
fn decode_value_format(reader: &mut Reader<'_>) -> Decoded<ValueFormat> {
    let position = reader.offset();
    let bits = reader.u16()?;
    ValueFormat::from_bits(bits).ok_or_else(|| {
        layout_error(
            position,
            LayoutCause::ReservedGposSingleValueFormat { format: bits },
        )
    })
}

/// Reads one value record's retained adjustments, consuming any device-table offsets that follow.
fn decode_value_record(reader: &mut Reader<'_>, format: ValueFormat) -> Decoded<ValueRecord> {
    let x_placement = read_adjustment(reader, format.has_x_placement())?;
    let y_placement = read_adjustment(reader, format.has_y_placement())?;
    let x_advance = read_adjustment(reader, format.has_x_advance())?;
    let y_advance = read_adjustment(reader, format.has_y_advance())?;
    let adjustment_bytes = usize::from(format.has_x_placement())
        + usize::from(format.has_y_placement())
        + usize::from(format.has_x_advance())
        + usize::from(format.has_y_advance());
    // Device and variation-index tables carry no retained adjustment; skip their offsets so the
    // cursor reaches the next record.
    reader.skip(format.byte_size() - adjustment_bytes * 2)?;
    Ok(ValueRecord::new(
        x_placement,
        y_placement,
        x_advance,
        y_advance,
    ))
}

/// Reads a signed adjustment when the format selects it, defaulting an absent field to zero.
fn read_adjustment(reader: &mut Reader<'_>, present: bool) -> Decoded<i16> {
    if present { reader.i16() } else { Ok(0) }
}

/// Resolves the Coverage offset and decodes the Coverage table it targets.
fn decode_coverage(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    minimum_position: usize,
) -> Decoded<Coverage> {
    let target = relative_target_span(
        owner,
        position,
        offset,
        minimum_position - owner.start(),
        LayoutTarget::SinglePositioningCoverage,
    )?;
    Coverage::decode(&bytes[..owner.end()], target.start())
}
