//! Iterative tri-color postorder traversal of the reachable component graph.

use glyphweave_sfnt::{Cause, Decoded, Diagnostic, GlyphId, OutlineCause, Stage};

use super::GlyphClosure;
use crate::{Glyf, GlyphSet};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Mark {
    #[default]
    Unseen,
    Active,
    Complete,
}

#[derive(Clone, Copy, Debug)]
struct Frame {
    glyph: GlyphId,
    next_component: usize,
    depth: u16,
}

impl Frame {
    const fn new(glyph: GlyphId) -> Self {
        Self {
            glyph,
            next_component: 0,
            depth: 0,
        }
    }
}

pub(super) fn compute(initial: &GlyphSet, glyf: &Glyf) -> Decoded<GlyphClosure> {
    let mut traversal = Traversal {
        glyf,
        glyphs: initial.clone(),
        marks: vec![Mark::Unseen; glyf.glyph_count()],
        depths: vec![0; glyf.glyph_count()],
        maximum_component_depth: 0,
    };
    for root in initial.iter() {
        if glyf.outline(root).is_none() {
            return Err(failure(OutlineCause::GlyphClosureRootOutOfBounds {
                glyph: root,
                glyph_count: glyf.glyph_count(),
            }));
        }
        traversal.visit(root)?;
    }
    Ok(GlyphClosure {
        glyphs: traversal.glyphs,
        maximum_component_depth: traversal.maximum_component_depth,
    })
}

struct Traversal<'a> {
    glyf: &'a Glyf,
    glyphs: GlyphSet,
    marks: Vec<Mark>,
    depths: Vec<u16>,
    maximum_component_depth: u16,
}

impl Traversal<'_> {
    fn visit(&mut self, root: GlyphId) -> Decoded<()> {
        let root_index = index(root);
        if self.marks[root_index] == Mark::Complete {
            self.maximum_component_depth =
                self.maximum_component_depth.max(self.depths[root_index]);
            return Ok(());
        }
        self.marks[root_index] = Mark::Active;
        let mut stack = vec![Frame::new(root)];

        while let Some(frame) = stack.last_mut() {
            let components = self
                .glyf
                .outline(frame.glyph)
                .and_then(crate::Outline::as_composite)
                .map_or(&[][..], crate::CompositeGlyph::components);
            if let Some(component) = components.get(frame.next_component) {
                frame.next_component += 1;
                let parent = frame.glyph;
                let child = component.glyph_id();
                if self.glyf.outline(child).is_none() {
                    return Err(failure(OutlineCause::CompositeComponentOutOfBounds {
                        parent,
                        component: child,
                        glyph_count: self.glyf.glyph_count(),
                    }));
                }
                self.glyphs.insert(child);
                let child_mark = self
                    .marks
                    .get(index(child))
                    .copied()
                    .expect("an existing outline has a traversal mark");
                match child_mark {
                    Mark::Unseen => {
                        self.marks[index(child)] = Mark::Active;
                        stack.push(Frame::new(child));
                    }
                    Mark::Active => {
                        return Err(failure(OutlineCause::CompositeGlyphCycle {
                            parent,
                            component: child,
                        }));
                    }
                    Mark::Complete => {
                        frame.depth = frame.depth.max(edge_depth(self.depths[index(child)]));
                    }
                }
            } else {
                let completed = stack.pop().expect("the loop observed a final frame");
                let completed_index = index(completed.glyph);
                self.marks[completed_index] = Mark::Complete;
                self.depths[completed_index] = completed.depth;
                if let Some(parent) = stack.last_mut() {
                    parent.depth = parent.depth.max(edge_depth(completed.depth));
                } else {
                    self.maximum_component_depth =
                        self.maximum_component_depth.max(completed.depth);
                }
            }
        }
        Ok(())
    }
}

const fn index(glyph: GlyphId) -> usize {
    glyph.raw() as usize
}

const fn edge_depth(child_depth: u16) -> u16 {
    child_depth.saturating_add(1)
}

const fn failure(cause: OutlineCause) -> Diagnostic {
    Diagnostic::failure(Stage::Closure, Cause::Outline(cause))
}
