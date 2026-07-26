//! Malformed-input boundaries for semantic GSUB decoding.

use glyphweave_layout::SingleSubstitution;
use glyphweave_sfnt::ByteSpan;

#[test]
fn every_strict_single_delta_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x06, 0x00, 0x01, // SingleSubst delta +1
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(SingleSubstitution::decode(&valid[..end], span).is_err());
    }
}
