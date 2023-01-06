//! One canonical child reference inside a composite glyph.

use glyphweave_sfnt::GlyphId;

use crate::{ComponentPlacement, ComponentTransform};

/// A referenced child glyph with placement and transform semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Component {
    glyph_id: GlyphId,
    placement: ComponentPlacement,
    transform: ComponentTransform,
    use_my_metrics: bool,
}

impl Component {
    /// Constructs a canonical component from already validated fields.
    #[must_use]
    pub const fn new(
        glyph_id: GlyphId,
        placement: ComponentPlacement,
        transform: ComponentTransform,
        use_my_metrics: bool,
    ) -> Self {
        Self {
            glyph_id,
            placement,
            transform,
            use_my_metrics,
        }
    }

    /// Returns the child Glyph ID referenced by this component.
    #[must_use]
    pub const fn glyph_id(self) -> GlyphId {
        self.glyph_id
    }
}
