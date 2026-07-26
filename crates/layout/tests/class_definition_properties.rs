//! Deterministic properties for Class Definition canonicalization and round trips.

use glyphweave_layout::ClassDefinition;
use glyphweave_sfnt::GlyphId;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x434c_4153),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn arbitrary_bytes_and_offsets_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..513),
        offset in any::<usize>(),
    ) {
        if let Ok(classes) = ClassDefinition::decode(&bytes, offset) {
            prop_assert!(classes.assignments().all(|(_, class)| class != 0));
            prop_assert!(classes
                .assignments()
                .map(|(glyph, _)| glyph)
                .is_sorted());
        }
    }

    #[test]
    fn canonical_values_round_trip(
        assignments in collection::vec((any::<u16>(), any::<u16>()), 0..257),
    ) {
        let classes = ClassDefinition::from_assignments(
            assignments
                .into_iter()
                .map(|(glyph, class)| (GlyphId::new(glyph), class)),
        );
        let encoded = classes.encode();
        let decoded = ClassDefinition::decode(&encoded, 0).unwrap();
        prop_assert_eq!(decoded, classes);
    }

    #[test]
    fn encoder_always_chooses_the_shortest_legal_format(
        assignments in collection::vec((any::<u16>(), any::<u16>()), 0..257),
    ) {
        let classes = ClassDefinition::from_assignments(
            assignments
                .into_iter()
                .map(|(glyph, class)| (GlyphId::new(glyph), class)),
        );
        let ordered: Vec<_> = classes.assignments().collect();
        let mut runs = 0usize;
        let mut previous: Option<(GlyphId, u16)> = None;
        for &(glyph, class) in &ordered {
            if !matches!(previous, Some((prior, value))
                if prior.raw().checked_add(1) == Some(glyph.raw()) && value == class)
            {
                runs += 1;
            }
            previous = Some((glyph, class));
        }
        let format_2_size = 4 + 6 * runs;
        let expected = ordered.first().zip(ordered.last()).map_or(format_2_size, |(first, last)| {
            let span = usize::from(last.0.raw()) - usize::from(first.0.raw()) + 1;
            u16::try_from(span).map_or(format_2_size, |_| format_2_size.min(6 + 2 * span))
        });
        prop_assert_eq!(classes.encode().len(), expected);
    }
}
