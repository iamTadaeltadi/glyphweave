//! Bounded TTC Header and Face decoding.

use super::FontCollection;
use crate::cause::Cause;
use crate::{
    ByteSpan, CollectionSignature, CollectionVersion, Decoded, Diagnostic, Face, FaceIndex, Reader,
    Stage, TableDirectory, Tag,
};

const TTC_TAG: Tag = Tag::new(*b"ttcf");
const DSIG_RAW: u32 = u32::from_be_bytes(*b"DSIG");

pub(super) fn decode(bytes: &[u8]) -> Decoded<FontCollection> {
    let mut reader = Reader::new(bytes);
    let tag = reader.tag()?;
    if tag != TTC_TAG {
        return Err(Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(0, 4).unwrap_or_else(|| ByteSpan::empty_at(0)),
            Cause::InvalidCollectionTag { found: tag },
        ));
    }
    let major = reader.u16()?;
    let minor = reader.u16()?;
    let version = CollectionVersion::from_parts(major, minor).ok_or_else(|| {
        Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(4, 4).unwrap_or_else(|| ByteSpan::empty_at(4)),
            Cause::UnsupportedCollectionVersion { major, minor },
        )
    })?;
    let count = reader.u32()?;
    if count == 0 {
        return Err(Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(8, 4).unwrap_or_else(|| ByteSpan::empty_at(8)),
            Cause::EmptyFontCollection,
        ));
    }
    let signature_field_length = match version {
        CollectionVersion::Version1 => 0,
        CollectionVersion::Version2 => 12,
    };
    let maximum = reader.remaining().saturating_sub(signature_field_length) / 4;
    let count_usize = usize::try_from(count).ok();
    if count_usize.is_none_or(|count| count > maximum) {
        return Err(Diagnostic::at(
            Stage::Decode,
            ByteSpan::sized(8, 4).unwrap_or_else(|| ByteSpan::empty_at(8)),
            Cause::CollectionFaceCountOutOfBounds { count, maximum },
        ));
    }
    let mut offsets = Vec::with_capacity(count_usize.unwrap_or(0));
    for _ in 0..count {
        offsets.push(reader.u32()?);
    }
    let signature = match version {
        CollectionVersion::Version1 => None,
        CollectionVersion::Version2 => decode_signature(bytes, &mut reader)?,
    };
    let header_span =
        ByteSpan::between(0, reader.offset()).unwrap_or_else(|| ByteSpan::empty_at(0));
    validate_unique_offsets(&offsets)?;
    let mut faces = Vec::with_capacity(offsets.len());
    for (raw_index, raw_offset) in offsets.into_iter().enumerate() {
        let directory_offset = usize::try_from(raw_offset).map_err(|_| {
            Diagnostic::at(
                Stage::Decode,
                header_span,
                Cause::SpanOutOfBounds {
                    end: usize::MAX,
                    length: bytes.len(),
                },
            )
        })?;
        if directory_offset < header_span.end() {
            let index = FaceIndex::new(u32::try_from(raw_index).unwrap_or(u32::MAX));
            let directory = ByteSpan::sized(directory_offset, 12)
                .unwrap_or_else(|| ByteSpan::empty_at(directory_offset));
            let field_start = 12usize.saturating_add(raw_index.saturating_mul(4));
            let field =
                ByteSpan::sized(field_start, 4).unwrap_or_else(|| ByteSpan::empty_at(field_start));
            return Err(Diagnostic::at(
                Stage::Decode,
                field,
                Cause::FaceDirectoryOverlapsCollectionHeader {
                    index,
                    directory,
                    header: header_span,
                },
            ));
        }
        let directory = TableDirectory::decode(bytes, directory_offset)?;
        faces.push(Face {
            index: FaceIndex::new(u32::try_from(raw_index).unwrap_or(u32::MAX)),
            directory_offset,
            directory,
        });
    }
    validate_directory_spans(&faces)?;
    validate_tables_outside_header(&faces, header_span)?;
    validate_tables_outside_directories(&faces)?;
    Ok(FontCollection {
        version,
        header_span,
        faces,
        signature,
    })
}

fn validate_tables_outside_directories(faces: &[Face]) -> Decoded<()> {
    for face in faces {
        for record in face.directory.records() {
            for directory_face in faces {
                let directory = directory_face.directory.span();
                if record.table_span().overlaps(directory) {
                    return Err(Diagnostic::at(
                        Stage::Decode,
                        record.table_span().join(directory),
                        Cause::TableOverlapsFaceDirectory {
                            face: face.index,
                            tag: record.tag(),
                            table: record.table_span(),
                            directory_face: directory_face.index,
                            directory,
                        },
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_tables_outside_header(faces: &[Face], header: ByteSpan) -> Decoded<()> {
    for face in faces {
        for record in face.directory.records() {
            if record.table_span().overlaps(header) {
                return Err(Diagnostic::at(
                    Stage::Decode,
                    record.table_span(),
                    Cause::TableOverlapsCollectionHeader {
                        face: face.index,
                        tag: record.tag(),
                        table: record.table_span(),
                        header,
                    },
                ));
            }
        }
    }
    Ok(())
}

fn validate_directory_spans(faces: &[Face]) -> Decoded<()> {
    let mut by_offset: Vec<_> = faces.iter().collect();
    by_offset.sort_unstable_by_key(|face| {
        (
            face.directory.span().start(),
            face.directory.span().end(),
            face.index,
        )
    });
    for pair in by_offset.windows(2) {
        let [first, second] = pair else { continue };
        let first_span = first.directory.span();
        let second_span = second.directory.span();
        if first_span.overlaps(second_span) {
            return Err(Diagnostic::at(
                Stage::Decode,
                first_span.join(second_span),
                Cause::OverlappingFaceDirectories {
                    first: (first.index, first_span),
                    second: (second.index, second_span),
                },
            ));
        }
    }
    Ok(())
}

fn validate_unique_offsets(offsets: &[u32]) -> Decoded<()> {
    let mut indexed: Vec<_> = offsets.iter().copied().enumerate().collect();
    indexed.sort_unstable_by_key(|&(index, offset)| (offset, index));
    for pair in indexed.windows(2) {
        let [(first_index, first_offset), (second_index, second_offset)] = pair else {
            continue;
        };
        if first_offset == second_offset {
            let first = FaceIndex::new(u32::try_from(*first_index).unwrap_or(u32::MAX));
            let second = FaceIndex::new(u32::try_from(*second_index).unwrap_or(u32::MAX));
            let field_start = 12usize.saturating_add(second_index.saturating_mul(4));
            let span =
                ByteSpan::sized(field_start, 4).unwrap_or_else(|| ByteSpan::empty_at(field_start));
            return Err(Diagnostic::at(
                Stage::Decode,
                span,
                Cause::DuplicateFaceDirectoryOffset {
                    first,
                    second,
                    offset: usize::try_from(*first_offset).unwrap_or(usize::MAX),
                },
            ));
        }
    }
    Ok(())
}

fn decode_signature(bytes: &[u8], reader: &mut Reader<'_>) -> Decoded<Option<CollectionSignature>> {
    let descriptor_start = reader.offset();
    let tag = reader.u32()?;
    let length = reader.u32()?;
    let offset = reader.u32()?;
    if (tag, length, offset) == (0, 0, 0) {
        return Ok(None);
    }
    let descriptor = ByteSpan::sized(descriptor_start, 12)
        .unwrap_or_else(|| ByteSpan::empty_at(descriptor_start));
    if tag != DSIG_RAW || length == 0 || offset % 4 != 0 {
        return Err(Diagnostic::at(
            Stage::Decode,
            descriptor,
            Cause::InvalidCollectionSignatureDescriptor {
                tag,
                length,
                offset,
            },
        ));
    }
    let span = usize::try_from(offset)
        .ok()
        .zip(usize::try_from(length).ok())
        .and_then(|(offset, length)| ByteSpan::sized(offset, length))
        .ok_or_else(|| {
            Diagnostic::at(
                Stage::Decode,
                descriptor,
                Cause::InvalidCollectionSignatureDescriptor {
                    tag,
                    length,
                    offset,
                },
            )
        })?;
    if span.end() != bytes.len() {
        return Err(Diagnostic::at(
            Stage::Decode,
            span,
            Cause::CollectionSignatureNotAtEnd {
                signature: span,
                font_length: bytes.len(),
            },
        ));
    }
    Ok(Some(CollectionSignature { span }))
}
