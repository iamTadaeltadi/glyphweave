//! Bounded decoding for both GPOS `PairPos` formats.

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LayoutCause, LayoutTarget, Reader};

use super::{PairAdjustment, PairPositioning, PairValue};
use crate::{
    ClassDefinition, Coverage, ValueFormat, ValueRecord, layout_error, relative_target_span,
};

impl PairPositioning {
    /// Decodes a `PairPos` subtable at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// For truncation, an unsupported format, reserved value-format bits, invalid offsets,
    /// Coverage, Class Definitions, or a `PairSet` count mismatch.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        match reader.u16()? {
            1 => Self::decode_format_1(bytes, span, &mut reader),
            2 => Self::decode_format_2(bytes, span, &mut reader),
            format => Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposPairFormat { format },
            )),
        }
    }

    fn decode_format_1(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let coverage_position = reader.offset();
        let coverage_offset = reader.u16()?;
        let value_format_1 = read_value_format(reader, 1)?;
        let value_format_2 = read_value_format(reader, 2)?;
        let count_position = reader.offset();
        let set_count = reader.u16()?;
        let mut offsets = Vec::with_capacity(usize::from(set_count));
        for _ in 0..set_count {
            let position = reader.offset();
            offsets.push((position, reader.u16()?));
        }
        let minimum_position = reader.offset();
        let coverage = decode_coverage(
            bytes,
            span,
            coverage_position,
            coverage_offset,
            minimum_position,
        )?;
        if coverage.len() != offsets.len() {
            return Err(layout_error(
                count_position,
                LayoutCause::MismatchedGposPairSetCount {
                    inputs: coverage.len(),
                    sets: offsets.len(),
                },
            ));
        }

        let mut sets = Vec::with_capacity(offsets.len());
        for (position, offset) in offsets {
            let target = relative_target_span(
                span,
                position,
                offset,
                minimum_position - span.start(),
                LayoutTarget::PairSet,
            )?;
            sets.push(decode_pair_set(
                bytes,
                target,
                value_format_1,
                value_format_2,
            )?);
        }
        Ok(Self::Glyphs { coverage, sets })
    }

    fn decode_format_2(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let coverage_position = reader.offset();
        let coverage_offset = reader.u16()?;
        let value_format_1 = read_value_format(reader, 1)?;
        let value_format_2 = read_value_format(reader, 2)?;
        let first_class_position = reader.offset();
        let first_class_offset = reader.u16()?;
        let second_class_position = reader.offset();
        let second_class_offset = reader.u16()?;
        let class1_count = reader.u16()?;
        let class2_count = reader.u16()?;

        let mut grid = Vec::with_capacity(usize::from(class1_count));
        for _ in 0..class1_count {
            let mut row = Vec::with_capacity(usize::from(class2_count));
            for _ in 0..class2_count {
                let first = read_value_record(reader, value_format_1)?;
                let second = read_value_record(reader, value_format_2)?;
                row.push(PairAdjustment::new(first, second));
            }
            grid.push(row);
        }

        let minimum_position = reader.offset();
        let coverage = decode_coverage(
            bytes,
            span,
            coverage_position,
            coverage_offset,
            minimum_position,
        )?;
        let first_classes = decode_classes(
            bytes,
            span,
            first_class_position,
            first_class_offset,
            minimum_position,
        )?;
        let second_classes = decode_classes(
            bytes,
            span,
            second_class_position,
            second_class_offset,
            minimum_position,
        )?;
        Ok(Self::Classes {
            coverage,
            first_classes,
            second_classes,
            grid,
        })
    }
}

/// Reads a value format, rejecting one that sets reserved bits.
fn read_value_format(reader: &mut Reader<'_>, record: u8) -> Decoded<ValueFormat> {
    let position = reader.offset();
    let bits = reader.u16()?;
    ValueFormat::from_bits(bits).ok_or_else(|| {
        layout_error(
            position,
            LayoutCause::ReservedGposPairValueFormat {
                record,
                format: bits,
            },
        )
    })
}

/// Reads one value record, consuming present adjustments and skipping present device offsets.
fn read_value_record(reader: &mut Reader<'_>, format: ValueFormat) -> Decoded<ValueRecord> {
    let x_placement = read_adjustment(reader, format.has_x_placement())?;
    let y_placement = read_adjustment(reader, format.has_y_placement())?;
    let x_advance = read_adjustment(reader, format.has_x_advance())?;
    let y_advance = read_adjustment(reader, format.has_y_advance())?;
    let value_fields = usize::from(format.has_x_placement())
        + usize::from(format.has_y_placement())
        + usize::from(format.has_x_advance())
        + usize::from(format.has_y_advance());
    reader.skip(format.byte_size() - value_fields * 2)?;
    Ok(ValueRecord::new(
        x_placement,
        y_placement,
        x_advance,
        y_advance,
    ))
}

/// Reads a present adjustment or yields zero for an absent field.
fn read_adjustment(reader: &mut Reader<'_>, present: bool) -> Decoded<i16> {
    if present { reader.i16() } else { Ok(0) }
}

/// Decodes one `PairSet`: a length-prefixed array of trailing glyphs and their paired adjustments.
fn decode_pair_set(
    bytes: &[u8],
    span: ByteSpan,
    first: ValueFormat,
    second: ValueFormat,
) -> Decoded<Vec<PairValue>> {
    let mut reader = Reader::within(bytes, span)?;
    let count = reader.u16()?;
    let mut values = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let trailing = GlyphId::new(reader.u16()?);
        let first_record = read_value_record(&mut reader, first)?;
        let second_record = read_value_record(&mut reader, second)?;
        values.push(PairValue::new(
            trailing,
            PairAdjustment::new(first_record, second_record),
        ));
    }
    Ok(values)
}

/// Resolves and decodes the Coverage table referenced by a `PairPos` header.
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
        LayoutTarget::PairPositioningCoverage,
    )?;
    Coverage::decode(&bytes[..owner.end()], target.start())
}

/// Resolves and decodes one Class Definition table referenced by a `PairPos` format 2 header.
fn decode_classes(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    minimum_position: usize,
) -> Decoded<ClassDefinition> {
    let target = relative_target_span(
        owner,
        position,
        offset,
        minimum_position - owner.start(),
        LayoutTarget::PairPositioningClassDefinition,
    )?;
    ClassDefinition::decode(&bytes[..owner.end()], target.start())
}
