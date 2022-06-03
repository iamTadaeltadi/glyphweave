use super::Cause;

#[test]
fn invalid_collection_tag_names_the_found_bytes() {
    let cause = Cause::InvalidCollectionTag {
        found: crate::Tag::new(*b"OTTO"),
    };
    assert_eq!(
        cause.to_string(),
        "expected collection tag ttcf but found Tag(OTTO)"
    );
}

#[test]
fn unsupported_collection_version_preserves_both_parts() {
    let cause = Cause::UnsupportedCollectionVersion { major: 2, minor: 1 };
    assert_eq!(cause.to_string(), "unsupported collection version 2.1");
}

#[test]
fn empty_collection_failure_is_explicit() {
    assert_eq!(
        Cause::EmptyFontCollection.to_string(),
        "font collection contains no faces"
    );
}

#[test]
fn face_index_failure_names_available_count() {
    let cause = Cause::FaceIndexOutOfBounds {
        index: crate::FaceIndex::new(4),
        face_count: 2,
    };
    assert_eq!(
        cause.to_string(),
        "face index 4 is out of bounds for 2 face(s)"
    );
}

#[test]
fn collection_count_failure_names_available_capacity() {
    let cause = Cause::CollectionFaceCountOutOfBounds {
        count: 100,
        maximum: 3,
    };
    assert_eq!(
        cause.to_string(),
        "collection face count 100 exceeds available offset capacity 3"
    );
}

#[test]
fn signature_descriptor_failure_preserves_raw_fields() {
    let cause = Cause::InvalidCollectionSignatureDescriptor {
        tag: 0x4241_4421,
        length: 12,
        offset: 41,
    };
    assert_eq!(
        cause.to_string(),
        "invalid collection signature descriptor tag=0x42414421 length=12 offset=41"
    );
}

#[test]
fn signature_position_failure_names_the_file_end() {
    let cause = Cause::CollectionSignatureNotAtEnd {
        signature: crate::ByteSpan::sized(40, 12).unwrap(),
        font_length: 60,
    };
    assert_eq!(
        cause.to_string(),
        "collection signature at 40..52 must end at file length 60"
    );
}

#[test]
fn signature_header_overlap_names_both_ranges() {
    let cause = Cause::CollectionSignatureOverlapsHeader {
        signature: crate::ByteSpan::sized(16, 32).unwrap(),
        header: crate::ByteSpan::sized(0, 28).unwrap(),
    };
    assert_eq!(
        cause.to_string(),
        "collection signature at 16..48 overlaps collection header 0..28"
    );
}

#[test]
fn duplicate_directory_failure_names_both_faces() {
    let cause = Cause::DuplicateFaceDirectoryOffset {
        first: crate::FaceIndex::new(1),
        second: crate::FaceIndex::new(3),
        offset: 64,
    };
    assert_eq!(
        cause.to_string(),
        "face indices 1 and 3 share directory offset 64"
    );
}

#[test]
fn collection_header_overlap_names_the_face() {
    let cause = Cause::FaceDirectoryOverlapsCollectionHeader {
        index: crate::FaceIndex::new(1),
        directory: crate::ByteSpan::sized(16, 12).unwrap(),
        header: crate::ByteSpan::sized(0, 20).unwrap(),
    };
    assert_eq!(
        cause.to_string(),
        "face 1 directory at 16..28 overlaps collection header 0..20"
    );
}

#[test]
fn overlapping_directory_failure_names_both_faces() {
    let cause = Cause::OverlappingFaceDirectories {
        first: (
            crate::FaceIndex::FIRST,
            crate::ByteSpan::sized(20, 28).unwrap(),
        ),
        second: (
            crate::FaceIndex::new(1),
            crate::ByteSpan::sized(40, 12).unwrap(),
        ),
    };
    assert_eq!(
        cause.to_string(),
        "face 0 directory at 20..48 overlaps face 1 directory at 40..52"
    );
}

#[test]
fn table_collection_header_overlap_names_owning_face() {
    let cause = Cause::TableOverlapsCollectionHeader {
        face: crate::FaceIndex::new(2),
        tag: crate::Tag::new(*b"name"),
        table: crate::ByteSpan::sized(8, 12).unwrap(),
        header: crate::ByteSpan::sized(0, 16).unwrap(),
    };
    assert_eq!(
        cause.to_string(),
        "face 2 table name at 8..20 overlaps collection header 0..16"
    );
}

#[test]
fn table_face_directory_overlap_names_both_faces() {
    let cause = Cause::TableOverlapsFaceDirectory {
        face: crate::FaceIndex::FIRST,
        tag: crate::Tag::new(*b"cmap"),
        table: crate::ByteSpan::sized(48, 20).unwrap(),
        directory_face: crate::FaceIndex::new(1),
        directory: crate::ByteSpan::sized(60, 12).unwrap(),
    };
    assert_eq!(
        cause.to_string(),
        "face 0 table cmap at 48..68 overlaps face 1 directory 60..72"
    );
}

#[test]
fn table_signature_overlap_names_the_face_and_table() {
    let cause = Cause::TableOverlapsCollectionSignature {
        face: crate::FaceIndex::new(3),
        tag: crate::Tag::new(*b"name"),
        table: crate::ByteSpan::sized(80, 20).unwrap(),
        signature: crate::ByteSpan::sized(96, 16).unwrap(),
    };
    assert_eq!(
        cause.to_string(),
        "face 3 table name at 80..100 overlaps collection signature 96..112"
    );
}

#[test]
fn cross_face_table_overlap_names_both_records() {
    let cause = Cause::OverlappingFaceTables {
        first: (
            crate::FaceIndex::FIRST,
            crate::Tag::new(*b"glyf"),
            crate::ByteSpan::sized(100, 40).unwrap(),
        ),
        second: (
            crate::FaceIndex::new(1),
            crate::Tag::new(*b"glyf"),
            crate::ByteSpan::sized(120, 40).unwrap(),
        ),
    };
    assert_eq!(
        cause.to_string(),
        "face 0 table glyf at 100..140 overlaps face 1 table glyf at 120..160"
    );
}

#[test]
fn unsupported_version_preserves_all_four_bytes() {
    let cause = Cause::UnsupportedSfntVersion { raw: 0x7472_7565 };
    assert_eq!(cause.to_string(), "unsupported SFNT version 0x74727565");
}

#[test]
fn table_count_failure_names_the_limit() {
    let cause = Cause::UnrepresentableTableCount {
        count: 4_096,
        maximum: 4_095,
    };
    assert_eq!(
        cause.to_string(),
        "table count 4096 exceeds representable maximum 4095"
    );
}

#[test]
fn search_parameter_failure_shows_both_triplets() {
    let cause = Cause::InvalidSearchParameters {
        expected: [128, 3, 80],
        actual: [64, 2, 144],
    };
    assert_eq!(
        cause.to_string(),
        "directory search parameters [64, 2, 144] do not match derived values [128, 3, 80]"
    );
}

#[test]
fn table_bounds_failure_names_the_record() {
    let cause = Cause::TableOutOfBounds {
        tag: crate::Tag::new(*b"glyf"),
        offset: 64,
        length: 40,
        font_length: 96,
    };
    assert_eq!(
        cause.to_string(),
        "table glyf at 64+40 exceeds font length 96"
    );
}

#[test]
fn invalid_tag_failure_uses_unambiguous_debug_form() {
    let cause = Cause::InvalidTableTag {
        tag: crate::Tag::new(*b"a b "),
    };
    assert_eq!(cause.to_string(), "invalid table tag Tag(a b )");
}

#[test]
fn duplicate_tag_failure_names_the_tag() {
    let cause = Cause::DuplicateTableTag {
        tag: crate::Tag::new(*b"cmap"),
    };
    assert_eq!(cause.to_string(), "duplicate table tag cmap");
}

#[test]
fn unsorted_tag_failure_names_the_adjacent_pair() {
    let cause = Cause::UnsortedTableTags {
        previous: crate::Tag::new(*b"head"),
        current: crate::Tag::new(*b"cmap"),
    };
    assert_eq!(cause.to_string(), "table tag cmap follows larger tag head");
}

#[test]
fn alignment_failure_names_table_and_offset() {
    let cause = Cause::MisalignedTable {
        tag: crate::Tag::new(*b"head"),
        offset: 29,
    };
    assert_eq!(
        cause.to_string(),
        "table head starts at unaligned offset 29"
    );
}

#[test]
fn directory_overlap_failure_names_both_ranges() {
    let cause = Cause::TableOverlapsDirectory {
        tag: crate::Tag::new(*b"cmap"),
        table: crate::ByteSpan::between(24, 40).unwrap(),
        directory: crate::ByteSpan::between(0, 28).unwrap(),
    };
    assert_eq!(
        cause.to_string(),
        "table cmap at 24..40 overlaps directory 0..28"
    );
}

#[test]
fn table_overlap_failure_names_both_records() {
    let cause = Cause::OverlappingTables {
        first: (
            crate::Tag::new(*b"cmap"),
            crate::ByteSpan::between(48, 64).unwrap(),
        ),
        second: (
            crate::Tag::new(*b"head"),
            crate::ByteSpan::between(60, 72).unwrap(),
        ),
    };
    assert_eq!(
        cause.to_string(),
        "table cmap at 48..64 overlaps table head at 60..72"
    );
}

#[test]
fn checksum_failure_renders_fixed_width_hex() {
    let cause = Cause::InvalidTableChecksum {
        tag: crate::Tag::new(*b"name"),
        declared: crate::Checksum::new(1),
        computed: crate::Checksum::new(0x1234),
    };
    assert_eq!(
        cause.to_string(),
        "table name checksum 0x00000001 does not match 0x00001234"
    );
}
