//! Cycle-safe transitive retention of composite components.

mod compute;

use glyphweave_sfnt::{Decoded, GlyphId};

use crate::{Glyf, GlyphSet};

/// A closed Glyph Set and its deepest reachable composite-component path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyphClosure {
    pub(crate) glyphs: GlyphSet,
    pub(crate) maximum_component_depth: u16,
}

impl GlyphClosure {
    /// Computes the reachable composite-component closure of an initial Glyph Set.
    ///
    /// ```
    /// use glyphweave_outline::{Glyf, GlyphClosure, GlyphSet, Loca};
    /// use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};
    ///
    /// let bytes = [0; 6];
    /// let loca = Loca::decode(
    ///     &bytes,
    ///     ByteSpan::sized(0, 6).unwrap(),
    ///     ByteSpan::empty_at(6),
    ///     2,
    ///     LocaFormat::Short,
    /// )?;
    /// let glyf = Glyf::decode(&bytes, &loca)?;
    /// let initial: GlyphSet = [GlyphId::new(1)].into_iter().collect();
    /// let closure = GlyphClosure::compute(&initial, &glyf)?;
    /// assert_eq!(closure.iter().map(GlyphId::raw).collect::<Vec<_>>(), [0, 1]);
    /// # Ok::<(), glyphweave_sfnt::Diagnostic>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a Closure-stage Diagnostic when a retained root or reachable component is outside
    /// `glyf`, or when a reachable component edge forms a cycle.
    pub fn compute(initial: &GlyphSet, glyf: &Glyf) -> Decoded<Self> {
        compute::compute(initial, glyf)
    }

    /// Visits closed Glyph IDs in ascending input order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = GlyphId> + DoubleEndedIterator + '_ {
        self.glyphs.iter()
    }

    /// Returns the greatest number of component edges from a retained root to a leaf.
    #[must_use]
    pub const fn maximum_component_depth(&self) -> u16 {
        self.maximum_component_depth
    }

    /// Returns whether a Glyph ID belongs to the transitive closure.
    #[must_use]
    pub fn contains(&self, glyph: GlyphId) -> bool {
        self.glyphs.contains(glyph)
    }

    /// Returns the number of retained Glyph IDs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Returns whether the closure contains no Glyph IDs.
    ///
    /// This is always `false` because every valid closure retains `.notdef`.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Borrows the complete retained Glyph Set.
    #[must_use]
    pub const fn glyphs(&self) -> &GlyphSet {
        &self.glyphs
    }
}
