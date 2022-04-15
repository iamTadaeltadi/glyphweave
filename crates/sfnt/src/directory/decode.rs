//! Bounded decoding of Table Directory headers and records.

use super::{TableDirectory, TableRecord};
use crate::cause::Cause;
use crate::search::{MAX_TABLES, SearchParameters};
use crate::{ByteSpan, Decoded, Diagnostic, Reader, SfntVersion, Stage};

pub(super) fn decode(bytes: &[u8], offset: usize) -> Decoded<TableDirectory> {
    let mut reader = Reader::at(bytes, offset)?;
    let raw_version = reader.u32()?;
    let version = SfntVersion::from_raw(raw_version).ok_or_else(|| {
        Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(offset, 4).unwrap_or_else(|| ByteSpan::empty_at(offset)),
            Cause::UnsupportedSfntVersion { raw: raw_version },
        )
    })?;

    let table_count = reader.u16()?;
    let expected = SearchParameters::derive(table_count).ok_or_else(|| {
        Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(offset + 4, 2).unwrap_or_else(|| ByteSpan::empty_at(offset + 4)),
            Cause::UnrepresentableTableCount {
                count: table_count,
                maximum: MAX_TABLES,
            },
        )
    })?;
    let actual = SearchParameters {
        search_range: reader.u16()?,
        entry_selector: reader.u16()?,
        range_shift: reader.u16()?,
    };
    if actual != expected {
        return Err(Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(offset + 6, 6).unwrap_or_else(|| ByteSpan::empty_at(offset + 6)),
            Cause::InvalidSearchParameters {
                expected: expected.into_array(),
                actual: actual.into_array(),
            },
        ));
    }

    let mut records: Vec<TableRecord> = Vec::with_capacity(usize::from(table_count));
    for _ in 0..table_count {
        let record = decode_record(bytes, &mut reader)?;
        if let Some(previous) = records.last() {
            let cause = match previous.tag.cmp(&record.tag) {
                core::cmp::Ordering::Equal => Some(Cause::DuplicateTableTag { tag: record.tag }),
                core::cmp::Ordering::Greater => Some(Cause::UnsortedTableTags {
                    previous: previous.tag,
                    current: record.tag,
                }),
                core::cmp::Ordering::Less => None,
            };
            if let Some(cause) = cause {
                return Err(Diagnostic::at(
                    Stage::Decode,
                    ByteSpan::sized(record.directory_span.start(), 4)
                        .unwrap_or_else(|| ByteSpan::empty_at(record.directory_span.start())),
                    cause,
                ));
            }
        }
        records.push(record);
    }
    let span =
        ByteSpan::between(offset, reader.offset()).unwrap_or_else(|| ByteSpan::empty_at(offset));
    Ok(TableDirectory {
        version,
        span,
        records,
    })
}

fn decode_record(bytes: &[u8], reader: &mut Reader<'_>) -> Decoded<TableRecord> {
    let record_start = reader.offset();
    let tag = reader.tag()?;
    if !tag.is_well_formed() {
        return Err(Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(record_start, 4).unwrap_or_else(|| ByteSpan::empty_at(record_start)),
            Cause::InvalidTableTag { tag },
        ));
    }
    let checksum = reader.u32()?;
    let offset = reader.u32()?;
    let length = reader.u32()?;
    let table_span = usize::try_from(offset)
        .ok()
        .zip(usize::try_from(length).ok())
        .and_then(|(start, length)| ByteSpan::sized(start, length))
        .filter(|span| span.end() <= bytes.len());
    let Some(table_span) = table_span else {
        return Err(Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(record_start, 16).unwrap_or_else(|| ByteSpan::empty_at(record_start)),
            Cause::TableOutOfBounds {
                tag,
                offset,
                length,
                font_length: bytes.len(),
            },
        ));
    };
    Ok(TableRecord {
        tag,
        checksum,
        table_span,
        directory_span: ByteSpan::sized(record_start, 16)
            .unwrap_or_else(|| ByteSpan::empty_at(record_start)),
    })
}
