//! Deterministic properties for bounded big-endian reads.

use glyphweave_sfnt::{Cause, Reader};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x5245_4144),
        ..Config::default()
    }
}

fn reader_request() -> impl Strategy<Value = (Vec<u8>, usize, usize)> {
    collection::vec(any::<u8>(), 0..129).prop_flat_map(|bytes| {
        let offsets = 0..=bytes.len();
        (Just(bytes), offsets, 0usize..160)
    })
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn array_reads_match_slice_bounds((bytes, offset, length) in reader_request()) {
        let mut reader = Reader::at(&bytes, offset).unwrap();
        let before = reader.offset();
        let observed = reader.array(length);
        if length <= bytes.len() - offset {
            prop_assert_eq!(observed.unwrap(), &bytes[offset..offset + length]);
            prop_assert_eq!(reader.offset(), offset + length);
        } else {
            let diagnostic = observed.unwrap_err();
            prop_assert_eq!(
                diagnostic.cause(),
                Cause::UnexpectedEnd {
                    needed: length,
                    available: bytes.len() - offset,
                },
            );
            prop_assert_eq!(reader.offset(), before);
        }
    }

    #[test]
    fn scalar_reads_preserve_big_endian_values(
        byte in any::<u8>(),
        word in any::<u16>(),
        signed in any::<i16>(),
        double_word in any::<u32>(),
        tag_bytes in any::<[u8; 4]>(),
    ) {
        let mut bytes = vec![byte];
        bytes.extend_from_slice(&word.to_be_bytes());
        bytes.extend_from_slice(&signed.to_be_bytes());
        bytes.extend_from_slice(&double_word.to_be_bytes());
        bytes.extend_from_slice(&tag_bytes);

        let mut reader = Reader::new(&bytes);
        prop_assert_eq!(reader.u8().unwrap(), byte);
        prop_assert_eq!(reader.u16().unwrap(), word);
        prop_assert_eq!(reader.i16().unwrap(), signed);
        prop_assert_eq!(reader.u32().unwrap(), double_word);
        prop_assert_eq!(reader.tag().unwrap().to_bytes(), tag_bytes);
        prop_assert!(reader.is_empty());
    }
}
