//! Byte-boundary tests for bounded Coverage decoding.

use glyphweave_layout::Coverage;
use glyphweave_sfnt::Cause;

#[test]
fn every_strict_format_1_prefix_is_rejected() {
    let complete = [
        0x00, 0x01, // format
        0x00, 0x03, // glyphCount
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03,
    ];

    for end in 0..complete.len() {
        let diagnostic = Coverage::decode(&complete[..end], 0).unwrap_err();
        assert!(
            matches!(diagnostic.cause(), Cause::UnexpectedEnd { .. }),
            "prefix {end} produced {}",
            diagnostic.cause()
        );
        assert!(diagnostic.span().is_some());
    }

    assert_eq!(Coverage::decode(&complete, 0).unwrap().len(), 3);
}

#[test]
fn a_truncated_glyph_reports_the_remaining_byte() {
    let bytes = [0x00, 0x01, 0x00, 0x01, 0x12];
    let diagnostic = Coverage::decode(&bytes, 0).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
    assert_eq!(diagnostic.span().unwrap().start(), 4);
}
