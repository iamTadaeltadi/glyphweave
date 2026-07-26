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

#[test]
fn loca_start_failure_names_the_leading_gap() {
    let cause = Cause::Outline(OutlineCause::NonzeroLocaStart { offset: 4 });

    assert_eq!(
        cause.to_string(),
        "loca first offset 4 does not start glyf at zero"
    );
}

#[test]
fn decreasing_loca_failure_names_both_offsets() {
    let cause = Cause::Outline(OutlineCause::DecreasingLocaOffset {
        index: 2,
        offset: 4,
        previous: 8,
    });

    assert_eq!(
        cause.to_string(),
        "loca offset 2 value 4 precedes previous offset 8"
    );
}

#[test]
fn misaligned_loca_failure_names_the_entry_and_offset() {
    let cause = Cause::Outline(OutlineCause::MisalignedLocaOffset {
        index: 1,
        offset: 3,
    });

    assert_eq!(
        cause.to_string(),
        "loca offset 1 value 3 is not word aligned"
    );
}
