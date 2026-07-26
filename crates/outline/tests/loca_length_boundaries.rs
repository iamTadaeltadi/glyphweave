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
