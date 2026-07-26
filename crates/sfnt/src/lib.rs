//! Bounded decoding primitives for SFNT containers, faces, table directories, and scalar tables.
//!
//! This crate is the dependency root of Glyphweave's computational pipeline. It performs no I/O
//! and exposes no unchecked view into input bytes. Every value decoded here is produced by the
//! bounded [`Reader`], located by a [`ByteSpan`], and reported through a span-aware
//! [`Diagnostic`].

#![forbid(unsafe_code)]

mod cause;
mod character_map;
mod checksum;
mod cmap_format12;
mod cmap_format4;
mod collection;
mod collection_signature;
mod collection_version;
mod diagnostic;
mod directory;
mod face;
mod face_index;
mod font_artifact;
mod glyph_id;
mod reader;
mod search;
mod span;
mod tag;
mod variation_selector;
mod version;

pub use cause::{Cause, CmapCause};
pub use character_map::CharacterMap;
pub use checksum::Checksum;
pub use cmap_format4::CmapFormat4;
pub use cmap_format12::CmapFormat12;
pub use collection::FontCollection;
pub use collection_signature::CollectionSignature;
pub use collection_version::CollectionVersion;
pub use diagnostic::{Decoded, Diagnostic, Severity, Stage};
pub use directory::{TableDirectory, TableRecord};
pub use face::Face;
pub use face_index::FaceIndex;
pub use font_artifact::FontArtifact;
pub use glyph_id::{GlyphId, NewGlyphId};
pub use reader::Reader;
pub use span::ByteSpan;
pub use tag::Tag;
pub use variation_selector::VariationSelector;
pub use version::SfntVersion;
