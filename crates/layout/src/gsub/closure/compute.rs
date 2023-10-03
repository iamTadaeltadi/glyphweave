//! Iterative GSUB glyph and Lookup reachability.

use std::collections::BTreeSet;

use glyphweave_sfnt::{Decoded, GlyphId, LayoutCause};

use super::{GsubClosure, apply};
use crate::{GsubLookupList, closure_error};

impl GsubClosure {
    /// Computes the smallest fixed point containing the initial glyphs and root Lookups.
    /// # Errors
    /// Returns a Closure-stage diagnostic when a root or contextual Lookup index is out of range.
    pub fn compute(
        list: &GsubLookupList,
        initial_glyphs: impl IntoIterator<Item = GlyphId>,
        root_lookups: impl IntoIterator<Item = u16>,
    ) -> Decoded<Self> {
        let mut glyphs = initial_glyphs.into_iter().collect::<BTreeSet<_>>();
        let mut lookups = BTreeSet::new();
        for index in root_lookups {
            validate_lookup(index, list.len())?;
            lookups.insert(index);
        }

        let mut passes = 0;
        loop {
            let before = (glyphs.len(), lookups.len());
            let reachable = lookups.iter().copied().collect::<Vec<_>>();
            for index in reachable {
                let lookup = require_lookup(list, index)?;
                apply::lookup(lookup, &mut glyphs, &mut lookups);
            }
            for &index in &lookups {
                validate_lookup(index, list.len())?;
            }
            if before == (glyphs.len(), lookups.len()) {
                break;
            }
            passes += 1;
        }
        Ok(Self {
            glyphs,
            lookups,
            passes,
        })
    }
}

fn require_lookup(list: &GsubLookupList, index: u16) -> Decoded<&crate::GsubLookup> {
    list.lookup(index).ok_or_else(|| {
        closure_error(LayoutCause::InvalidContextLookupIndex {
            lookup_index: index,
            lookup_count: list.len(),
        })
    })
}

fn validate_lookup(index: u16, lookup_count: usize) -> Decoded<()> {
    if usize::from(index) < lookup_count {
        Ok(())
    } else {
        Err(closure_error(LayoutCause::InvalidContextLookupIndex {
            lookup_index: index,
            lookup_count,
        }))
    }
}
