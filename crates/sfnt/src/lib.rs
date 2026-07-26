//! Bounded decoding primitives for SFNT containers, faces, table directories, and scalar tables.
//!
//! This crate is the dependency root of Glyphweave's computational pipeline. It performs no I/O
//! and exposes no unchecked view into input bytes. Every value decoded here is produced by the
//! bounded [`Reader`], located by a [`ByteSpan`], and reported through a span-aware
//! [`Diagnostic`].

#![forbid(unsafe_code)]

mod cause;
mod checksum;
mod collection;
mod collection_signature;
mod collection_version;
mod diagnostic;
mod directory;
mod face;
mod face_index;
mod glyph_id;
mod reader;
mod search;
mod span;
mod tag;
mod version;

pub use cause::Cause;
pub use checksum::Checksum;
pub use collection::FontCollection;
pub use collection_signature::CollectionSignature;
pub use collection_version::CollectionVersion;
pub use diagnostic::{Decoded, Diagnostic, Severity, Stage};
pub use directory::{TableDirectory, TableRecord};
pub use face::Face;
pub use face_index::FaceIndex;
pub use glyph_id::GlyphId;
pub use reader::Reader;
pub use span::ByteSpan;
pub use tag::Tag;
pub use version::SfntVersion;
