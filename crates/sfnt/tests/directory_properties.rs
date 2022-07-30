//! Deterministic generated SFNT Table Directories.

use glyphweave_sfnt::{ByteSpan, Cause, Checksum, SfntVersion, TableDirectory, Tag};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

#[derive(Clone, Debug)]
struct GeneratedFont {
    bytes: Vec<u8>,
    tables: Vec<(Tag, Vec<u8>)>,
}

fn fixed_config() -> Config {
    Config {
        cases: 128,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4449_5245),
        ..Config::default()
    }
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn search_parameters(count: u16) -> [u16; 3] {
    if count == 0 {
        return [0; 3];
    }
    let selector = u16::try_from(u16::BITS - 1 - count.leading_zeros()).unwrap();
    let power = 1u16 << selector;
    [power * 16, selector, count * 16 - power * 16]
}

fn generated_font() -> impl Strategy<Value = GeneratedFont> {
    collection::vec(collection::vec(any::<u8>(), 0..65), 0..33).prop_map(|payloads| {
        let count = u16::try_from(payloads.len()).unwrap();
        let directory_length = 12 + payloads.len() * 16;
        let mut offset = directory_length;
        let tables = payloads
            .into_iter()
            .enumerate()
            .map(|(index, payload)| {
                let tens = u8::try_from(index / 10).unwrap();
                let ones = u8::try_from(index % 10).unwrap();
                let tag = Tag::new([b'T', b'0' + tens, b'0' + ones, b' ']);
                let table_offset = offset;
                offset += (payload.len() + 3) & !3;
                (tag, payload, table_offset)
            })
            .collect::<Vec<_>>();

        let mut bytes = Vec::with_capacity(offset);
        push_u32(&mut bytes, 0x0001_0000);
        push_u16(&mut bytes, count);
        for parameter in search_parameters(count) {
            push_u16(&mut bytes, parameter);
        }
        for (tag, payload, table_offset) in &tables {
            bytes.extend_from_slice(&tag.to_bytes());
            push_u32(&mut bytes, Checksum::compute(payload).raw());
            push_u32(&mut bytes, u32::try_from(*table_offset).unwrap());
            push_u32(&mut bytes, u32::try_from(payload.len()).unwrap());
        }
        for (_, payload, _) in &tables {
            bytes.extend_from_slice(payload);
            bytes.resize((bytes.len() + 3) & !3, 0);
        }
        GeneratedFont {
            bytes,
            tables: tables
                .into_iter()
                .map(|(tag, payload, _)| (tag, payload))
                .collect(),
        }
    })
}

fn generated_font_with_payload() -> impl Strategy<Value = GeneratedFont> {
    generated_font().prop_filter("at least one nonempty table", |font| {
        font.tables.iter().any(|(_, payload)| !payload.is_empty())
    })
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_directories_decode_to_their_source_model(font in generated_font()) {
        let directory = TableDirectory::decode(&font.bytes, 0).unwrap();
        prop_assert_eq!(directory.version(), SfntVersion::TrueType);
        prop_assert_eq!(directory.span(), ByteSpan::sized(0, 12 + font.tables.len() * 16).unwrap());
        prop_assert_eq!(directory.len(), font.tables.len());
        for ((expected_tag, expected_bytes), record) in font.tables.iter().zip(directory.records()) {
            prop_assert_eq!(record.tag(), *expected_tag);
            prop_assert_eq!(record.checksum(), Checksum::compute(expected_bytes));
            prop_assert_eq!(record.bytes(&font.bytes).unwrap(), expected_bytes);
            prop_assert_eq!(record.table_span().start() % 4, 0);
            prop_assert_eq!(directory.record(*expected_tag), Some(record));
        }
        for (index, first) in directory.records().iter().enumerate() {
            prop_assert!(first.table_span().end() <= font.bytes.len());
            prop_assert!(!first.table_span().overlaps(directory.span()));
            for second in &directory.records()[index + 1..] {
                prop_assert!(!first.table_span().overlaps(second.table_span()));
            }
        }
    }


    #[test]
    fn payload_corruption_is_always_detected(
        mut font in generated_font_with_payload(),
        selector in any::<usize>(),
    ) {
        let directory = TableDirectory::decode(&font.bytes, 0).unwrap();
        let records = directory
            .records()
            .iter()
            .copied()
            .filter(|record| !record.table_span().is_empty())
            .collect::<Vec<_>>();
        let record = records[selector % records.len()];
        font.bytes[record.table_span().start()] ^= 0xff;

        let diagnostic = TableDirectory::decode(&font.bytes, 0).unwrap_err();
        let computed = Checksum::compute(record.table_span().slice(&font.bytes).unwrap());
        prop_assert_eq!(
            diagnostic.cause(),
            Cause::InvalidTableChecksum {
                tag: record.tag(),
                declared: record.checksum(),
                computed,
            },
        );
    }
}
