//! TrueType outline decoding, composite glyph closure, remapping, and rewriting.
//!
//! Outline transformations are pure and refer to source locations through checked SFNT byte
//! spans.

#![forbid(unsafe_code)]

mod glyph_set;
mod remap;

pub use glyph_set::GlyphSet;
pub use remap::Remap;
