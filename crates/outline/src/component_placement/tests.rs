use super::{ComponentOffsetMode, ComponentPlacement};

#[test]
fn offset_mode_defaults_to_platform_compatible_behavior() {
    assert_eq!(ComponentOffsetMode::default(), ComponentOffsetMode::Default);
}

#[test]
fn point_and_offset_placements_remain_distinct() {
    let offset = ComponentPlacement::Offset {
        x: 7,
        y: -9,
        round_to_grid: true,
        mode: ComponentOffsetMode::Scaled,
    };
    let points = ComponentPlacement::MatchPoints {
        parent: 7,
        child: 9,
    };

    assert_ne!(offset, points);
}
