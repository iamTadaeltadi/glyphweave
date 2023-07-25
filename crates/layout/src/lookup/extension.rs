//! Validation and flattening for GSUB and GPOS extension wrappers.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTableKind, LayoutTarget, Reader};

use crate::{layout_error_sized, relative_target_span_u32};

#[derive(Debug)]
pub(super) struct DecodedExtension {
    pub(super) lookup_type: u16,
    pub(super) type_position: usize,
    pub(super) payload: ByteSpan,
}

pub(super) fn is_extension_type(table: LayoutTableKind, lookup_type: u16) -> bool {
    matches!(
        (table, lookup_type),
        (LayoutTableKind::Substitution, 7) | (LayoutTableKind::Positioning, 9)
    )
}

pub(super) fn is_valid_type(table: LayoutTableKind, lookup_type: u16) -> bool {
    match table {
        LayoutTableKind::Substitution => (1..=8).contains(&lookup_type),
        LayoutTableKind::Positioning => (1..=9).contains(&lookup_type),
        _ => false,
    }
}

pub(super) fn decode_extension(
    bytes: &[u8],
    span: ByteSpan,
    table: LayoutTableKind,
) -> Decoded<DecodedExtension> {
    let mut reader = Reader::within(bytes, span)?;
    let format_position = reader.offset();
    let format = reader.u16()?;
    if format != 1 {
        return Err(layout_error_sized(
            format_position,
            2,
            LayoutCause::UnsupportedExtensionFormat { table, format },
        ));
    }
    let type_position = reader.offset();
    let lookup_type = reader.u16()?;
    if is_extension_type(table, lookup_type) {
        return Err(layout_error_sized(
            type_position,
            2,
            LayoutCause::RecursiveExtensionLookupType { table, lookup_type },
        ));
    }
    if !is_valid_type(table, lookup_type) {
        return Err(layout_error_sized(
            type_position,
            2,
            LayoutCause::InvalidLookupType { table, lookup_type },
        ));
    }
    let offset_position = reader.offset();
    let offset = reader.u32()?;
    let payload = relative_target_span_u32(
        span,
        offset_position,
        4,
        offset,
        reader.offset() - span.start(),
        LayoutTarget::ExtensionSubtable,
    )?;
    Ok(DecodedExtension {
        lookup_type,
        type_position,
        payload,
    })
}

pub(super) fn flatten_extensions(
    bytes: &[u8],
    wrappers: Vec<ByteSpan>,
    table: LayoutTableKind,
) -> Decoded<(u16, Vec<ByteSpan>)> {
    let mut effective_type = None;
    let mut payloads = Vec::with_capacity(wrappers.len());
    for wrapper in wrappers {
        let decoded = decode_extension(bytes, wrapper, table)?;
        if let Some(expected) = effective_type {
            if decoded.lookup_type != expected {
                return Err(crate::layout_error(
                    decoded.type_position,
                    LayoutCause::MismatchedExtensionLookupTypes {
                        table,
                        expected,
                        actual: decoded.lookup_type,
                    },
                ));
            }
        } else {
            effective_type = Some(decoded.lookup_type);
        }
        payloads.push(decoded.payload);
    }
    Ok((
        effective_type.expect("nonempty extension Lookup established its type"),
        payloads,
    ))
}

#[cfg(test)]
mod tests;
