# ADR 0014: Canonicalize composite components without resolving their graph

- Status: Accepted
- Date: 2026-07-26

## Context

A TrueType composite glyph stores a sequence of child Glyph IDs, placement arguments, optional
F2DOT14 transforms, and control flags. Several flag combinations encode the same effective value,
while component point alignment cannot be resolved until child outlines are available. The next
Glyph Closure stage also needs component references without interpreting outline geometry.

Keeping raw flag words would leak packed-format details into closure and rewriting. Expanding every
child outline during decoding would instead combine local syntax validation with recursive graph
resolution and cycle handling.

## Decision

Decode one composite description into a `CompositeGlyph` containing ordered `Component` values.
Each component stores its referenced Glyph ID, canonical placement, exact F2DOT14 transform, and
semantic metric behavior. Offset placement retains signed design-unit coordinates, grid-rounding,
and whether an offset is scaled, unscaled, or uses the platform-compatible default. Point placement
retains unsigned parent and child point indices without trying to resolve them.

Reject reserved flags, mutually exclusive transform encodings, conflicting offset modes, a
point-aligned first component, and a misplaced overlap marker. Preserve the parent instruction
program as opaque owned bytes. Accept any negative contour marker as a composite classification,
matching the common Glyph Header and the OpenType requirement that negative values identify the
composite format.

Decoding remains local to one checked Glyph Byte Span. It does not follow child references,
validate point indices against child outlines, expand contours, or detect component cycles. Those
graph-dependent checks belong to Glyph Closure in M2.7.

## Consequences

- Glyph Closure can traverse stable child Glyph IDs without re-parsing packed records.
- Canonical emission can choose an encoding independently from the source flag layout.
- Exact fixed-point transform bits remain available without floating-point equality or rounding.
- Point-reference and cycle validity are deliberately deferred until the complete glyph graph is
  available.

## Source

OpenType 1.9.1, [`glyf` composite glyph description](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf#composite-glyph-description).
