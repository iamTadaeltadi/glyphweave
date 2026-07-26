//! Deterministic properties for total `loca` decoding.

use glyphweave_outline::Loca;
use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};
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

    #[test]
    fn generated_location_arrays_match_their_glyph_span_model(
        half_increments in collection::vec(0_u16..=64, 0..65),
        prefix in collection::vec(any::<u8>(), 0..17),
        is_long in any::<bool>(),
    ) {
        let mut offsets = vec![0_u32];
        for increment in &half_increments {
            let next = offsets.last().copied().unwrap() + u32::from(*increment) * 2;
            offsets.push(next);
        }
        let format = if is_long { LocaFormat::Long } else { LocaFormat::Short };
        let mut bytes = prefix;
        let table_start = bytes.len();
        for offset in &offsets {
            match format {
                LocaFormat::Short => {
                    let encoded = u16::try_from(*offset / 2).unwrap();
                    bytes.extend_from_slice(&encoded.to_be_bytes());
                }
                LocaFormat::Long => bytes.extend_from_slice(&offset.to_be_bytes()),
            }
        }
        let table_span = ByteSpan::between(table_start, bytes.len()).unwrap();
        let glyf_start = bytes.len();
        let glyf_length = usize::try_from(*offsets.last().unwrap()).unwrap();
        bytes.resize(glyf_start + glyf_length, 0xaa);
        let glyf_span = ByteSpan::sized(glyf_start, glyf_length).unwrap();
        let glyph_count = u16::try_from(half_increments.len()).unwrap();

        let decoded = Loca::decode(&bytes, table_span, glyf_span, glyph_count, format).unwrap();
        prop_assert_eq!(decoded.offsets(), offsets.as_slice());
        for (index, pair) in offsets.windows(2).enumerate() {
            let glyph = GlyphId::new(u16::try_from(index).unwrap());
            let expected = ByteSpan::between(
                glyf_start + usize::try_from(pair[0]).unwrap(),
                glyf_start + usize::try_from(pair[1]).unwrap(),
            );
            prop_assert_eq!(decoded.glyph_span(glyph), expected);
        }
    }
}
