//! Bounded orchestration of simple-glyph logical streams.

mod coordinates;
mod flags;

use glyphweave_sfnt::{ByteSpan, Cause, Decoded, Diagnostic, OutlineCause, Reader, Stage};

use self::flags::ExpandedFlags;
use super::SimpleGlyph;
use crate::{GlyphBounds, GlyphHeader, Point};

const HEADER_LENGTH: usize = 10;
const CONTOUR_COUNT_LENGTH: usize = 2;
const BOUNDS_LENGTH: usize = 8;

pub(super) fn decode(bytes: &[u8], glyph_span: ByteSpan) -> Decoded<SimpleGlyph> {
    let header = GlyphHeader::decode(bytes, glyph_span)?;
    let contour_count = header.simple_contour_count().ok_or_else(|| {
        Diagnostic::at(
            Stage::Decode,
            field_span(glyph_span.start(), 0, CONTOUR_COUNT_LENGTH),
            Cause::Outline(OutlineCause::CompositeGlyphForSimpleDecoder {
                number_of_contours: header.number_of_contours(),
            }),
        )
    })?;

    let mut reader = Reader::within(bytes, glyph_span)?;
    reader.skip(HEADER_LENGTH)?;
    let contour_ends = decode_contour_ends(&mut reader, contour_count)?;
    let point_count = contour_ends.last().copied().unwrap_or(0);
    let instructions = decode_instructions(&mut reader, contour_count)?.to_vec();
    let expanded_flags = flags::decode(&mut reader, point_count)?;
    let x_coordinates = coordinates::decode_horizontal(&mut reader, &expanded_flags)?;
    let y_coordinates = coordinates::decode_vertical(&mut reader, &expanded_flags)?;
    let points = assemble_points(&expanded_flags, x_coordinates, y_coordinates);

    consume_alignment(&mut reader, glyph_span)?;
    verify_bounds(header.bounds(), &points, glyph_span)?;

    Ok(SimpleGlyph {
        span: glyph_span,
        bounds: header.bounds(),
        contour_ends,
        points,
        instructions,
        overlapping_contours: expanded_flags.overlapping_contours(),
    })
}

fn decode_contour_ends(reader: &mut Reader<'_>, count: u16) -> Decoded<Vec<u32>> {
    let mut ends = Vec::with_capacity(usize::from(count));
    let mut previous: Option<u16> = None;
    for contour_index in 0..count {
        let offset = reader.offset();
        let end_point = reader.u16()?;
        if let Some(previous_end_point) = previous {
            if end_point <= previous_end_point {
                return Err(Diagnostic::at(
                    Stage::Decode,
                    field_span(offset, 0, 2),
                    Cause::Outline(OutlineCause::NonIncreasingContourEnd {
                        contour_index,
                        end_point,
                        previous_end_point,
                    }),
                ));
            }
        }
        ends.push(u32::from(end_point) + 1);
        previous = Some(end_point);
    }
    Ok(ends)
}

fn decode_instructions<'a>(reader: &mut Reader<'a>, contour_count: u16) -> Decoded<&'a [u8]> {
    if contour_count == 0 && reader.is_empty() {
        return Ok(&[]);
    }
    let instruction_length = usize::from(reader.u16()?);
    reader.array(instruction_length)
}

fn assemble_points(
    flags: &ExpandedFlags,
    x_coordinates: Vec<i16>,
    y_coordinates: Vec<i16>,
) -> Vec<Point> {
    flags
        .iter()
        .zip(x_coordinates)
        .zip(y_coordinates)
        .map(|((flag, x), y)| Point::new(x, y, flag.is_on_curve()))
        .collect()
}

fn consume_alignment(reader: &mut Reader<'_>, glyph_span: ByteSpan) -> Decoded<()> {
    match reader.remaining() {
        0 => Ok(()),
        1 => {
            let offset = reader.offset();
            let value = reader.u8()?;
            if value == 0 {
                Ok(())
            } else {
                Err(Diagnostic::at(
                    Stage::Decode,
                    field_span(offset, 0, 1),
                    Cause::Outline(OutlineCause::InvalidSimpleGlyphPadding { value }),
                ))
            }
        }
        length => Err(Diagnostic::at(
            Stage::Decode,
            ByteSpan::between(reader.offset(), glyph_span.end()).unwrap_or(glyph_span),
            Cause::Outline(OutlineCause::UnexpectedSimpleGlyphData { length }),
        )),
    }
}

fn verify_bounds(recorded: GlyphBounds, points: &[Point], glyph_span: ByteSpan) -> Decoded<()> {
    let Some(calculated) = calculate_bounds(points) else {
        return Ok(());
    };
    if recorded == calculated {
        return Ok(());
    }
    Err(Diagnostic::at(
        Stage::Decode,
        field_span(glyph_span.start(), CONTOUR_COUNT_LENGTH, BOUNDS_LENGTH),
        Cause::Outline(OutlineCause::SimpleGlyphBoundsMismatch {
            recorded: bounds_array(recorded),
            calculated: bounds_array(calculated),
        }),
    ))
}

fn calculate_bounds(points: &[Point]) -> Option<GlyphBounds> {
    let first = *points.first()?;
    let mut minimum_x = first.x();
    let mut minimum_y = first.y();
    let mut maximum_x = first.x();
    let mut maximum_y = first.y();
    for point in &points[1..] {
        minimum_x = minimum_x.min(point.x());
        minimum_y = minimum_y.min(point.y());
        maximum_x = maximum_x.max(point.x());
        maximum_y = maximum_y.max(point.y());
    }
    GlyphBounds::new(minimum_x, minimum_y, maximum_x, maximum_y)
}

const fn bounds_array(bounds: GlyphBounds) -> [i16; 4] {
    [
        bounds.minimum_x(),
        bounds.minimum_y(),
        bounds.maximum_x(),
        bounds.maximum_y(),
    ]
}

fn field_span(start: usize, relative: usize, length: usize) -> ByteSpan {
    start
        .checked_add(relative)
        .and_then(|offset| ByteSpan::sized(offset, length))
        .unwrap_or_else(|| ByteSpan::empty_at(start))
}
