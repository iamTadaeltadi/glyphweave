//! Integration coverage for eager `glyf` decoding.

use glyphweave_outline::{Glyf, Loca};
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LocaFormat, Stage};

fn mixed_fixture() -> (Vec<u8>, Loca) {
    let mut bytes = vec![0, 0, 0, 0, 0, 5, 0, 13];
    bytes.extend([0; 10]);
    bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend([0x00, 0x02, 0x00, 0x01, 0x00, 0x00]);
    let loca = Loca::decode(
        &bytes,
        ByteSpan::sized(0, 8).unwrap(),
        ByteSpan::sized(8, 26).unwrap(),
        3,
        LocaFormat::Short,
    )
    .unwrap();
    (bytes, loca)
}

#[test]
fn equal_loca_offsets_preserve_empty_outlines() {
    let bytes = [0; 12];
    let loca_span = ByteSpan::sized(0, 12).unwrap();
    let glyf_span = ByteSpan::empty_at(12);
    let loca = Loca::decode(&bytes, loca_span, glyf_span, 2, LocaFormat::Long).unwrap();

    let glyf = Glyf::decode(&bytes, &loca).unwrap();

    assert_eq!(glyf.table_span(), glyf_span);
    assert_eq!(glyf.glyph_count(), 2);
    assert_eq!(glyf.outlines().len(), 2);
    assert!(glyf.outline(GlyphId::NOTDEF).unwrap().is_empty());
    assert!(glyf.outline(GlyphId::new(1)).unwrap().is_empty());
    assert_eq!(glyf.outline(GlyphId::new(2)), None);
}

#[test]
fn decodes_mixed_outlines_in_glyph_id_order() {
    let (bytes, loca) = mixed_fixture();

    let glyf = Glyf::decode(&bytes, &loca).unwrap();

    assert!(glyf.outline(GlyphId::NOTDEF).unwrap().is_empty());
    let simple = glyf.outline(GlyphId::new(1)).unwrap();
    assert_eq!(simple.span(), ByteSpan::between(8, 18).unwrap());
    assert_eq!(simple.as_simple().unwrap().contour_count(), 0);
    let composite = glyf.outline(GlyphId::new(2)).unwrap();
    assert_eq!(composite.span(), ByteSpan::between(18, 34).unwrap());
    assert_eq!(
        composite.as_composite().unwrap().components()[0].glyph_id(),
        GlyphId::new(1)
    );
}

#[test]
fn glyf_iteration_pairs_ids_with_ordered_outlines() {
    let (bytes, loca) = mixed_fixture();
    let glyf = Glyf::decode(&bytes, &loca).unwrap();

    let mut outlines = glyf.iter();

    assert_eq!(outlines.len(), 3);
    assert_eq!(outlines.next().unwrap().0, GlyphId::NOTDEF);
    assert_eq!(outlines.next().unwrap().0, GlyphId::new(1));
    assert_eq!(outlines.next().unwrap().0, GlyphId::new(2));
    assert_eq!(outlines.next(), None);
}

#[test]
fn eager_decode_rejects_a_malformed_later_glyph() {
    let mut bytes = vec![0, 0, 0, 5, 0, 6];
    bytes.extend([0; 10]);
    bytes.extend([0, 0]);
    let loca = Loca::decode(
        &bytes,
        ByteSpan::sized(0, 6).unwrap(),
        ByteSpan::sized(6, 12).unwrap(),
        2,
        LocaFormat::Short,
    )
    .unwrap();

    let diagnostic = Glyf::decode(&bytes, &loca).expect_err("glyph one has a truncated header");

    assert_eq!(diagnostic.stage(), Stage::Decode);
    assert_eq!(diagnostic.span(), Some(ByteSpan::empty_at(18)));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 0,
        }
    );
}

#[test]
fn eager_decode_reports_the_lowest_malformed_glyph_first() {
    let mut bytes = vec![0, 0, 0, 1, 0, 3];
    bytes.extend([0, 0]);
    bytes.extend([0, 0, 0, 0]);
    let loca = Loca::decode(
        &bytes,
        ByteSpan::sized(0, 6).unwrap(),
        ByteSpan::sized(6, 6).unwrap(),
        2,
        LocaFormat::Short,
    )
    .unwrap();

    let diagnostic = Glyf::decode(&bytes, &loca).unwrap_err();

    assert_eq!(diagnostic.span(), Some(ByteSpan::empty_at(8)));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 0,
        }
    );
}
