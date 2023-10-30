use glyphweave_sfnt::{GlyphId, NewGlyphId};

use super::direct;
use crate::{AlternateSubstitution, MultipleSubstitution};

#[test]
fn sequence_substitutions_preserve_output_order_and_deletion() {
    let multiple = MultipleSubstitution::from_mappings([
        (GlyphId::new(9), vec![GlyphId::new(12), GlyphId::new(4)]),
        (GlyphId::new(3), Vec::new()),
    ]);
    let alternate = AlternateSubstitution::from_alternates([(
        GlyphId::new(7),
        vec![GlyphId::new(10), GlyphId::new(8)],
    )]);
    let mut mapping = |glyph: GlyphId| Some(NewGlyphId::new(glyph.raw() + 20));

    let multiple = direct::multiple(&multiple, &mut mapping).unwrap();
    let alternate = direct::alternate(&alternate, &mut mapping).unwrap();

    assert_eq!(multiple.replacement(NewGlyphId::new(23)), Some(&[][..]));
    assert_eq!(
        multiple.replacement(NewGlyphId::new(29)),
        Some(&[NewGlyphId::new(32), NewGlyphId::new(24)][..])
    );
    assert_eq!(
        alternate.alternates(NewGlyphId::new(27)),
        Some(&[NewGlyphId::new(30), NewGlyphId::new(28)][..])
    );
}
