//! TrueType outline decoding, composite glyph closure, remapping, and rewriting.
//!
//! Outline transformations are pure and refer to source locations through checked SFNT byte
//! spans.

#![forbid(unsafe_code)]

mod component;
mod component_placement;
mod component_transform;
mod composite_glyph;
mod f2_dot_14;
mod glyph_bounds;
mod glyph_header;
mod glyph_set;
mod loca;
mod outline;
mod point;
mod remap;
mod simple_glyph;

pub use component::Component;
pub use component_placement::{ComponentOffsetMode, ComponentPlacement};
pub use component_transform::ComponentTransform;
pub use composite_glyph::CompositeGlyph;
pub use f2_dot_14::F2Dot14;
pub use glyph_bounds::GlyphBounds;
pub use glyph_header::GlyphHeader;
pub use glyph_set::GlyphSet;
pub use loca::Loca;
pub use outline::Outline;
pub use point::Point;
pub use remap::Remap;
pub use simple_glyph::SimpleGlyph;
