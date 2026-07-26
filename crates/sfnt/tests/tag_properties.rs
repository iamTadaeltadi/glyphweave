//! Deterministic properties for OpenType Tags.

use glyphweave_sfnt::Tag;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x5441_4701),
        ..Config::default()
    }
}

fn well_formed_tag() -> impl Strategy<Value = Tag> {
    collection::vec(0x21u8..=0x7e, 1..5).prop_map(|content| {
        let mut bytes = [b' '; 4];
        bytes[..content.len()].copy_from_slice(&content);
        Tag::new(bytes)
    })
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_tags_obey_the_opentype_grammar(tag in well_formed_tag()) {
        prop_assert!(tag.is_well_formed());
        let bytes = tag.to_bytes();
        prop_assert!(bytes
            .iter()
            .all(|byte| byte.is_ascii_graphic() || *byte == b' '));
        if let Some(first_space) = bytes.iter().position(|byte| *byte == b' ') {
            prop_assert!(bytes[first_space..].iter().all(|byte| *byte == b' '));
        }
    }


    #[test]
    fn arbitrary_tag_bits_round_trip(raw in any::<u32>()) {
        let tag = Tag::from_u32(raw);
        prop_assert_eq!(tag.to_u32(), raw);
        prop_assert_eq!(Tag::new(tag.to_bytes()), tag);
    }

    #[test]
    fn tag_order_matches_big_endian_integer_order(first in any::<u32>(), second in any::<u32>()) {
        prop_assert_eq!(
            Tag::from_u32(first).cmp(&Tag::from_u32(second)),
            first.cmp(&second),
        );
    }
}
