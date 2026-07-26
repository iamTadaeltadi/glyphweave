# ADR 0008: Eagerly resolve cmap format 4

- Status: Accepted
- Date: 2026-07-26

## Context

Format 4 stores Basic Multilingual Plane character mappings in parallel segment arrays. A segment
either applies a signed delta directly to character codes or uses an `idRangeOffset` whose base is
the address of that offset word itself. The latter rule makes a lazy view easy to mis-index and lets
malformed offsets point into headers or neighboring arrays.

Glyphweave's public Character Map represents Unicode scalar values with Rust `char`, and its
missing result is always `.notdef`. Later subset planning needs deterministic iteration, not the
original segment compression.

## Decision

Define `CmapFormat4` in `glyphweave-sfnt`. Decode it from complete immutable bytes plus an absolute
subtable offset. Record the declared Byte Span, language, segment count, and a canonical resolved
Character Map.

Validate format number, even declared length, minimum array capacity, nonzero even `segCountX2`,
derived search parameters, and zero `reservedPad`. Segment end values must increase strictly,
segments must not overlap, and each start must not exceed its end. The final segment must have both
start and end equal to `0xFFFF`.

Resolve every segment eagerly. A zero `idRangeOffset` applies the signed delta modulo 65,536. A
nonzero offset must be even, and every referenced word must lie at or after `glyphIdArray` and
within the declared subtable. A zero glyph-array word remains missing; nonzero words receive the
same wrapping delta.

Do not store explicit glyph-zero entries. Reject a nonzero mapping for a UTF-16 surrogate or for
the required `0xFFFF` sentinel because neither is a Unicode scalar value representable in the
resolved Character Map. Empty and missing mappings remain `.notdef` through total lookup.

## Consequences

- Consumers never repeat the address-relative range-offset calculation.
- Malformed offsets cannot observe bytes outside the declared subtable or reinterpret header data
  as glyph indices.
- Resolved mappings iterate in Unicode scalar order and compose directly with Codepoint Sets.
- Memory scales with the number of supported BMP scalars rather than encoded segment count.
- Format 12 can later merge supplementary mappings into the same Character Map abstraction.
- A later outer `cmap` decoder remains responsible for encoding-record precedence and language
  rules because format 4 alone does not identify its platform.
