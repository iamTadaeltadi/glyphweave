# ADR 0015: Compute composite closure with iterative postorder traversal

- Status: Accepted
- Date: 2026-07-26

## Context

Glyph Closure must add every transitively referenced composite component to an initial Glyph Set.
Composite graphs may share descendants, contain cycles, or form paths tens of thousands of glyphs
deep. A recursive depth-first traversal risks exhausting the native call stack before it can return
a Diagnostic. A breadth-first traversal finds membership but cannot distinguish an active back edge
from ordinary shared descendants or calculate exact nesting depth without another pass.

Closure also needs a stable graph boundary. Resolving components directly from raw `glyf` bytes
would repeat parsing and mix byte validation with graph validation.

## Decision

Eagerly decode every `loca` interval into a canonical `Glyf` collection of `Outline` values. Empty
intervals remain explicit empty outlines; non-empty intervals contain an already validated Simple
Glyph or Composite Glyph. Glyph IDs index this immutable collection directly.

Compute closure from the requested Glyph Set with an explicit stack and three visit states:
unseen, active, and complete. Visit roots in ascending Glyph ID order and components in source order.
An edge to an active glyph is a cycle; an edge outside `Glyf` is dangling. Complete each node in
postorder and memoize its maximum distance to a leaf, so shared descendants are traversed once and
the closure records the exact maximum component depth.

Only the graph reachable from the initial Glyph Set participates in closure. Unselected graph
validation belongs to independent whole-font Verification. The traversal performs no outline
expansion, point alignment, transform arithmetic, or instruction interpretation.

## Consequences

- Closure memory is proportional to the reachable glyph count, with no native recursion.
- Cycles and dangling references produce deterministic Closure-stage Diagnostics.
- Empty component glyphs are retained normally and terminate a path like simple outlines.
- The measured depth can later be compared with `maxp.maxComponentDepth` independently.
- Eager `Glyf` decoding supplies one stable boundary for closure, rewriting, and Verification.
