# ADR 0013: Canonicalize packed simple glyphs into checked points

- Status: Accepted
- Date: 2026-07-26

## Context

A TrueType simple glyph combines five independently bounded encodings: a shared ten-byte Glyph
Header, contour end-point indices, an instruction program, run-compressed point flags, and two
variable-width coordinate-delta streams. The last contour end determines the logical point count,
while each expanded flag determines how many bytes the later x and y streams consume. Malformed
counts can therefore redirect one logical stream into another without leaving the Glyph Byte Span.

Downstream closure and emission need points and contour boundaries, not the source compression.
Keeping flag runs and relative deltas in the domain model would spread stateful decoding rules into
Verification and encoding. The normative representation is defined by the OpenType 1.9.1
[`glyf`](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf) specification.

## Decision

Define `GlyphBounds`, `Point`, and `SimpleGlyph` in `glyphweave-outline`. `SimpleGlyph::decode`
accepts complete immutable font bytes and one checked, non-empty Glyph Byte Span obtained from
`Loca`. It rejects a span shorter than the common header and a negative `numberOfContours`, which
belongs to composite decoding in M2.6.

Represent contour ends as exclusive `u32` point indices. Require source `endPtsOfContours` values
to increase strictly, and derive the total point count from the last endpoint with checked
arithmetic. Preserve the instruction program as owned bytes without interpreting TrueType bytecode.
For a zero-contour glyph, accept the header alone; if bytes follow it, require the ordinary
instruction-length field and complete program.

Expand each physical flag and its optional repeat count into exactly one logical flag per point.
Reject the reserved high bit, a repeat that exceeds the point count, and `OVERLAP_SIMPLE` outside
the first physical flag. Store overlap as a glyph property rather than a point flag.

Decode the complete x stream before the y stream. Interpret short vectors and same-coordinate bits
according to their paired flag, accumulate deltas from zero with checked arithmetic, and require
every absolute coordinate to remain representable as an `i16`. Store canonical absolute points
with only position and on-curve identity.

Require ordered header bounds. For glyphs with points, require the recorded bounds to equal the
minimum and maximum decoded coordinates. Consume the complete Glyph Byte Span, except for one
optional zero byte when the packed description has odd length and the enclosing block is word
aligned. All failures use typed `OutlineCause` values and the narrowest responsible Byte Span.

## Consequences

- Consumers iterate contours and absolute points without understanding packed flags or deltas.
- Truncation in any logical stream fails before later bytes are reinterpreted.
- Coordinate overflow, impossible bounds, reserved flags, and surplus payload bytes cannot enter
  the validated model.
- Instruction bytes remain available for lossless retention and later canonical emission.
- Simple and composite glyphs can share `GlyphBounds` and the common header classification while
  retaining independent decoders.
