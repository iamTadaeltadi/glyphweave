//! Cycle-safe transitive retention of composite components.

use crate::GlyphSet;

/// A closed Glyph Set and its deepest reachable composite-component path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyphClosure {
    pub(crate) glyphs: GlyphSet,
    pub(crate) maximum_component_depth: u16,
}
