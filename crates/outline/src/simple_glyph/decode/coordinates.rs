//! Signed accumulation of variable-width coordinate deltas.

use glyphweave_sfnt::{
    ByteSpan, Cause, CoordinateAxis, Decoded, Diagnostic, OutlineCause, Reader, Stage,
};

use super::flags::{
    ExpandedFlags, PointFlag, X_SAME_OR_POSITIVE, X_SHORT_VECTOR, Y_SAME_OR_POSITIVE,
    Y_SHORT_VECTOR,
};

pub(super) fn decode_horizontal(
    reader: &mut Reader<'_>,
    flags: &ExpandedFlags,
) -> Decoded<Vec<i16>> {
    decode(reader, flags, CoordinateAxis::Horizontal)
}

pub(super) fn decode_vertical(reader: &mut Reader<'_>, flags: &ExpandedFlags) -> Decoded<Vec<i16>> {
    decode(reader, flags, CoordinateAxis::Vertical)
}

fn decode(
    reader: &mut Reader<'_>,
    flags: &ExpandedFlags,
    axis: CoordinateAxis,
) -> Decoded<Vec<i16>> {
    let mut coordinates = Vec::with_capacity(flags.len());
    let mut previous = 0i16;
    for (index, flag) in flags.iter().enumerate() {
        let offset = reader.offset();
        let delta = decode_delta(reader, flag, axis)?;
        let coordinate = previous.checked_add(delta).ok_or_else(|| {
            Diagnostic::at(
                Stage::Decode,
                coordinate_span(offset, reader.offset()),
                Cause::Outline(OutlineCause::SimpleCoordinateOverflow {
                    axis,
                    point_index: u32::try_from(index).unwrap_or(u32::MAX),
                    previous,
                    delta,
                }),
            )
        })?;
        coordinates.push(coordinate);
        previous = coordinate;
    }
    Ok(coordinates)
}

fn decode_delta(reader: &mut Reader<'_>, flag: PointFlag, axis: CoordinateAxis) -> Decoded<i16> {
    let (short_mask, same_or_positive_mask) = match axis {
        CoordinateAxis::Horizontal => (X_SHORT_VECTOR, X_SAME_OR_POSITIVE),
        CoordinateAxis::Vertical => (Y_SHORT_VECTOR, Y_SAME_OR_POSITIVE),
    };
    if flag.contains(short_mask) {
        let magnitude = i16::from(reader.u8()?);
        return Ok(if flag.contains(same_or_positive_mask) {
            magnitude
        } else {
            -magnitude
        });
    }
    if flag.contains(same_or_positive_mask) {
        Ok(0)
    } else {
        reader.i16()
    }
}

fn coordinate_span(start: usize, end: usize) -> ByteSpan {
    ByteSpan::between(start, end).unwrap_or_else(|| ByteSpan::empty_at(start))
}
