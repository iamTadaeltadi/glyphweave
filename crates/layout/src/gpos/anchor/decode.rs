//! Bounded decoding for OpenType Anchor formats 1, 2, and 3.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::Anchor;
use crate::{layout_error, relative_target_span};

/// The encoded size of a format 3 Anchor header and the first legal device-table offset.
const ANCHOR_FORMAT_3_SIZE: usize = 10;

impl Anchor {
    /// Decodes an Anchor table at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic for truncation, an unsupported anchor format, or a
    /// format 3 device offset that falls inside the header or outside the Anchor table's owner.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        match reader.u16()? {
            1 => {
                let x = reader.i16()?;
                let y = reader.i16()?;
                Ok(Self::coordinate(x, y))
            }
            2 => {
                let x = reader.i16()?;
                let y = reader.i16()?;
                let contour_point = reader.u16()?;
                Ok(Self::contour_point(x, y, contour_point))
            }
            3 => {
                let x = reader.i16()?;
                let y = reader.i16()?;
                let x_device = decode_device_offset(span, &mut reader)?;
                let y_device = decode_device_offset(span, &mut reader)?;
                Ok(Self::devices(x, y, x_device, y_device))
            }
            format => Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposAnchorFormat { format },
            )),
        }
    }
}

/// Reads one format 3 device offset, treating null as absent and bounds-checking the rest.
fn decode_device_offset(owner: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Option<u16>> {
    let position = reader.offset();
    let offset = reader.u16()?;
    if offset == 0 {
        return Ok(None);
    }
    relative_target_span(
        owner,
        position,
        offset,
        ANCHOR_FORMAT_3_SIZE,
        LayoutTarget::GposAnchorDevice,
    )?;
    Ok(Some(offset))
}
