use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use super::retain;
use crate::{AlternateSubstitution, GsubClosure, GsubSubtable, MultipleSubstitution};

#[test]
fn sequence_retention_preserves_deletion_and_filters_alternates() {
    let closure = closed([3, 4, 7]);
    let multiple = GsubSubtable::Multiple(MultipleSubstitution::from_mappings([
        (GlyphId::new(3), Vec::new()),
        (GlyphId::new(4), vec![GlyphId::new(9)]),
    ]));
    let alternate = GsubSubtable::Alternate(AlternateSubstitution::from_alternates([
        (GlyphId::new(3), vec![GlyphId::new(9), GlyphId::new(7)]),
        (GlyphId::new(8), vec![GlyphId::new(7)]),
    ]));

    let Some(GsubSubtable::Multiple(multiple)) = retain(&multiple, &closure) else {
        panic!("closed deletion must remain a MultipleSubst mapping");
    };
    let Some(GsubSubtable::Alternate(alternate)) = retain(&alternate, &closure) else {
        panic!("a closed alternate must remain an AlternateSubst mapping");
    };

    assert_eq!(multiple.replacement(GlyphId::new(3)), Some(&[][..]));
    assert_eq!(multiple.replacement(GlyphId::new(4)), None);
    assert_eq!(
        alternate.alternates(GlyphId::new(3)),
        Some(&[GlyphId::new(7)][..])
    );
    assert_eq!(alternate.alternates(GlyphId::new(8)), None);
}

fn closed(raw_glyphs: impl IntoIterator<Item = u16>) -> GsubClosure {
    GsubClosure {
        glyphs: raw_glyphs
            .into_iter()
            .map(GlyphId::new)
            .collect::<BTreeSet<_>>(),
        lookups: BTreeSet::new(),
        passes: 0,
    }
}
