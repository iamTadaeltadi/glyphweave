//! Offset-invariant coverage for TrueType glyph locations.

use glyphweave_outline::Loca;
use glyphweave_sfnt::{ByteSpan, Cause, LocaFormat, OutlineCause};

#[test]
fn first_offset_must_start_at_the_beginning_of_glyf() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&4_u32.to_be_bytes());
    bytes.extend_from_slice(&4_u32.to_be_bytes());
    bytes.extend_from_slice(&[0xaa; 4]);
    let table_span = ByteSpan::sized(0, 8).unwrap();
    let glyf_span = ByteSpan::sized(8, 4).unwrap();

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 1, LocaFormat::Long)
        .expect_err("leading glyf bytes are not assigned to glyph zero");

    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::NonzeroLocaStart { offset: 4 })
    );
}
