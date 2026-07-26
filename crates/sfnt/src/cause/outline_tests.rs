use super::{Cause, OutlineCause};
use crate::LocaFormat;

#[test]
fn loca_length_failure_names_format_count_and_lengths() {
    let cause = Cause::Outline(OutlineCause::InvalidLocaLength {
        format: LocaFormat::Short,
        glyph_count: 3,
        expected: 8,
        actual: 6,
    });

    assert_eq!(
        cause.to_string(),
        "Short loca length 6 does not equal 8 bytes required by 3 glyph(s)"
    );
}
