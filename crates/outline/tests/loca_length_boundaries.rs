//! Table-length coverage for TrueType glyph locations.

use glyphweave_outline::Loca;
use glyphweave_sfnt::{ByteSpan, Cause, LocaFormat, OutlineCause};

#[test]
fn short_table_must_hold_one_sentinel_beyond_every_glyph() {
    let bytes = [0_u8; 8];
    let table_span = ByteSpan::sized(0, 4).unwrap();
    let glyf_span = ByteSpan::sized(4, 4).unwrap();

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Short)
        .expect_err("three short entries need six bytes");

    assert_eq!(diagnostic.span(), Some(table_span));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::InvalidLocaLength {
            format: LocaFormat::Short,
            glyph_count: 2,
            expected: 6,
            actual: 4,
        })
    );
}

#[test]
fn long_table_cannot_advertise_surplus_location_entries() {
    let bytes = [0_u8; 16];
    let table_span = ByteSpan::sized(0, 16).unwrap();
    let glyf_span = ByteSpan::empty_at(16);

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Long)
        .expect_err("three long entries need exactly twelve bytes");

    assert_eq!(diagnostic.span(), Some(table_span));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::InvalidLocaLength {
            format: LocaFormat::Long,
            glyph_count: 2,
            expected: 12,
            actual: 16,
        })
    );
}

#[test]
fn loca_span_must_be_in_bounds_before_entries_are_observed() {
    let bytes = [0_u8; 5];
    let table_span = ByteSpan::sized(0, 6).unwrap();
    let glyf_span = ByteSpan::empty_at(5);

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Short)
        .expect_err("the declared loca span exceeds available bytes");

    assert_eq!(diagnostic.span(), Some(table_span));
    assert_eq!(
        diagnostic.cause(),
        Cause::SpanOutOfBounds { end: 6, length: 5 }
    );
}

#[test]
fn glyf_span_must_be_in_bounds_before_offsets_are_resolved() {
    let bytes = [0_u8; 3];
    let table_span = ByteSpan::sized(0, 2).unwrap();
    let glyf_span = ByteSpan::sized(2, 2).unwrap();

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 0, LocaFormat::Short)
        .expect_err("the paired glyf span exceeds available bytes");

    assert_eq!(diagnostic.span(), Some(glyf_span));
    assert_eq!(
        diagnostic.cause(),
        Cause::SpanOutOfBounds { end: 4, length: 3 }
    );
}
