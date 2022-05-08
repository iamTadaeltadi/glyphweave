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
