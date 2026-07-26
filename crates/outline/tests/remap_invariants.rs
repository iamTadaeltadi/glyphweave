//! Public invariants for canonical glyph remapping.

use glyphweave_outline::{GlyphSet, Remap};
use glyphweave_sfnt::{GlyphId, NewGlyphId};

#[test]
fn generated_glyph_sets_remap_totally_densely_and_reversibly() {
    for seed in 0u16..64 {
        let glyphs: GlyphSet = (0u16..1024)
            .filter(|raw| raw.wrapping_mul(31).wrapping_add(seed) % 17 == 0)
            .map(GlyphId::new)
            .collect();
        let remap = Remap::new(&glyphs);

        assert_eq!(remap.len(), glyphs.len());
        for (expected_new, (old, new)) in remap.iter().enumerate() {
            assert!(glyphs.contains(old));
            assert_eq!(usize::from(new.raw()), expected_new);
            assert_eq!(remap.new_glyph_id(old), Some(new));
            assert_eq!(remap.glyph_id(new), Some(old));
        }

        for raw in 0u16..1024 {
            let old = GlyphId::new(raw);
            assert_eq!(
                remap.new_glyph_id(old).is_some(),
                glyphs.contains(old),
                "membership mismatch for seed {seed}, glyph {raw}"
            );
        }

        let first_undefined = NewGlyphId::new(u16::try_from(remap.len()).unwrap_or(u16::MAX));
        assert_eq!(remap.glyph_id(first_undefined), None);
    }
}
