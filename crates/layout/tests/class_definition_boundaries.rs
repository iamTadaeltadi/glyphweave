//! Byte-boundary tests for bounded Class Definition decoding.

use glyphweave_layout::ClassDefinition;
use glyphweave_sfnt::Cause;

#[test]
fn every_strict_format_1_prefix_is_rejected() {
    let complete = [
        0x00, 0x01, // format
        0x00, 0x03, // startGlyphID
        0x00, 0x03, // glyphCount
        0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
    ];

    for end in 0..complete.len() {
        let diagnostic = ClassDefinition::decode(&complete[..end], 0).unwrap_err();
        assert!(
            matches!(diagnostic.cause(), Cause::UnexpectedEnd { .. }),
            "prefix {end} produced {}",
            diagnostic.cause()
        );
    }

    assert_eq!(
        ClassDefinition::decode(&complete, 0)
            .unwrap()
            .assignments()
            .len(),
        2
    );
}

#[test]
fn a_truncated_class_value_reports_the_remaining_byte() {
    let bytes = [0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x12];
    let diagnostic = ClassDefinition::decode(&bytes, 0).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
    assert_eq!(diagnostic.span().unwrap().start(), 6);
}

#[test]
fn every_strict_format_2_prefix_is_rejected() {
    let complete = [
        0x00, 0x02, // format
        0x00, 0x02, // classRangeCount
        0x00, 0x01, 0x00, 0x02, 0x00, 0x01, // first range
        0x00, 0x04, 0x00, 0x05, 0x00, 0x02, // second range
    ];

    for end in 0..complete.len() {
        let diagnostic = ClassDefinition::decode(&complete[..end], 0).unwrap_err();
        assert!(
            matches!(diagnostic.cause(), Cause::UnexpectedEnd { .. }),
            "prefix {end} produced {}",
            diagnostic.cause()
        );
    }

    assert_eq!(ClassDefinition::decode(&complete, 0).unwrap().len(), 4);
}
