//! Deterministic generated OpenType Font Collections.

use glyphweave_sfnt::{ByteSpan, CollectionVersion, Face, FaceIndex, FontCollection, SfntVersion};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

#[derive(Clone, Debug)]
struct GeneratedCollection {
    bytes: Vec<u8>,
    version: CollectionVersion,
    flavors: Vec<SfntVersion>,
}

fn fixed_config() -> Config {
    Config {
        cases: 128,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x5454_4346),
        ..Config::default()
    }
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn generated_collection() -> impl Strategy<Value = GeneratedCollection> {
    (any::<bool>(), collection::vec(any::<bool>(), 1..65)).prop_map(|(version_two, cff_faces)| {
        let version = if version_two {
            CollectionVersion::Version2
        } else {
            CollectionVersion::Version1
        };
        let header_length = if version_two { 24 } else { 12 } + cff_faces.len() * 4;
        let mut bytes = b"ttcf".to_vec();
        let (major, minor) = version.parts();
        push_u16(&mut bytes, major);
        push_u16(&mut bytes, minor);
        push_u32(&mut bytes, u32::try_from(cff_faces.len()).unwrap());
        for index in 0..cff_faces.len() {
            push_u32(
                &mut bytes,
                u32::try_from(header_length + index * 12).unwrap(),
            );
        }
        if version_two {
            bytes.extend_from_slice(&[0; 12]);
        }

        let flavors = cff_faces
            .into_iter()
            .map(|cff| {
                let flavor = if cff {
                    SfntVersion::OpenTypeCff
                } else {
                    SfntVersion::TrueType
                };
                push_u32(&mut bytes, flavor.raw());
                bytes.extend_from_slice(&[0; 8]);
                flavor
            })
            .collect();
        GeneratedCollection {
            bytes,
            version,
            flavors,
        }
    })
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_collections_preserve_every_face(
        generated in generated_collection(),
        selector in any::<usize>(),
    ) {
        let collection = FontCollection::decode(&generated.bytes).unwrap();
        let header_length = if generated.version == CollectionVersion::Version2 { 24 } else { 12 }
            + generated.flavors.len() * 4;
        prop_assert_eq!(collection.version(), generated.version);
        prop_assert_eq!(collection.header_span(), ByteSpan::sized(0, header_length).unwrap());
        prop_assert_eq!(collection.len(), generated.flavors.len());
        prop_assert_eq!(collection.signature(), None);
        for (index, (face, flavor)) in collection.faces().iter().zip(&generated.flavors).enumerate() {
            prop_assert_eq!(face.index(), FaceIndex::new(u32::try_from(index).unwrap()));
            prop_assert_eq!(face.directory_offset(), header_length + index * 12);
            prop_assert_eq!(face.directory().version(), *flavor);
        }

        let selected_index = selector % generated.flavors.len();
        let selected = Face::select(
            &generated.bytes,
            FaceIndex::new(u32::try_from(selected_index).unwrap()),
        )
        .unwrap();
        prop_assert_eq!(&selected, &collection.faces()[selected_index]);
    }
}
