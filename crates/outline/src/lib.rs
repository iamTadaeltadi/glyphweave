//! TrueType outline decoding, composite glyph closure, remapping, and rewriting.
//!
//! Outline transformations are pure and refer to source locations through checked SFNT byte
//! spans.

#![forbid(unsafe_code)]

mod f2_dot_14;
mod glyph_bounds;
mod glyph_header;
mod glyph_set;
mod loca;
mod point;
mod remap;
mod simple_glyph;

pub use f2_dot_14::F2Dot14;
pub use glyph_bounds::GlyphBounds;
pub use glyph_header::GlyphHeader;
pub use glyph_set::GlyphSet;
pub use loca::Loca;
pub use point::Point;
pub use remap::Remap;
pub use simple_glyph::SimpleGlyph;
