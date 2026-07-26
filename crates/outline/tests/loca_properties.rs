//! Deterministic properties for total `loca` decoding.

use glyphweave_outline::Loca;
use glyphweave_sfnt::{ByteSpan, LocaFormat};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4c4f_4341),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn arbitrary_bytes_spans_counts_and_formats_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..513),
        table_start in any::<usize>(),
        table_length in 0usize..513,
        glyf_start in any::<usize>(),
        glyf_length in 0usize..513,
        glyph_count in any::<u16>(),
        is_long in any::<bool>(),
    ) {
        let table_span = ByteSpan::sized(table_start, table_length)
            .unwrap_or_else(|| ByteSpan::empty_at(table_start));
        let glyf_span = ByteSpan::sized(glyf_start, glyf_length)
            .unwrap_or_else(|| ByteSpan::empty_at(glyf_start));
        let format = if is_long { LocaFormat::Long } else { LocaFormat::Short };

        if let Ok(decoded) = Loca::decode(
            &bytes,
            table_span,
            glyf_span,
            glyph_count,
            format,
        ) {
            prop_assert!(decoded.table_span().end() <= bytes.len());
            prop_assert!(decoded.glyf_span().end() <= bytes.len());
            prop_assert_eq!(decoded.offsets().len(), usize::from(glyph_count) + 1);
            prop_assert_eq!(decoded.offsets().first(), Some(&0));
            prop_assert!(decoded.offsets().windows(2).all(|pair| pair[0] <= pair[1]));
            prop_assert!(decoded.offsets().iter().all(|offset| offset % 2 == 0));
            prop_assert_eq!(
                decoded.offsets().last().and_then(|offset| usize::try_from(*offset).ok()),
                Some(glyf_length),
            );
        }
    }
}
