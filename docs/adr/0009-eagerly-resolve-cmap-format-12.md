# ADR 0009: Eagerly resolve cmap format 12

- Status: Accepted
- Date: 2026-07-26

## Context

Format 12 represents full-repertoire character mappings as ordered sequential groups. Each group
contains 32-bit start and end character codes plus a 32-bit starting glyph ID; later character
codes map to consecutive glyph IDs. Unlike format 4, its byte length is 32-bit and its records are
fixed-width, but both character and glyph arithmetic can exceed the domains represented by Rust
`char` and Glyphweave's `GlyphId`.

The OpenType specification requires groups to be sorted by increasing start character code and
requires each end to precede the next start. Glyphweave's downstream stages consume a canonical
Character Map rather than compressed group records.

## Decision

Define `CmapFormat12` in `glyphweave-sfnt`. Decode it from complete immutable bytes plus an absolute
subtable offset. Record the declared Byte Span, 32-bit language value, group count, and an eagerly
resolved Character Map.

Validate format number 12, zero reserved word, a declared length of at least 16 bytes, complete
availability, and exact equality between the declared length and `16 + numGroups * 12`. Empty group
arrays are valid. Reject inverted groups, groups that overlap or are not strictly ordered, and any
sequential glyph range that overflows `u32` or exceeds the 16-bit Glyph ID domain.

Resolve each group using checked arithmetic. Do not store explicit glyph-zero entries. Reject a
nonzero mapping whose character code is not a Unicode scalar value, including surrogate code units
and values above `0x10FFFF`. Record every failure against the header field or group Byte Span that
caused it.

Keep format 12 decoding independent of encoding records. A later outer `cmap` decoder will enforce
platform and encoding compatibility, language rules, and the required precedence of a 32-bit
Unicode subtable over a 16-bit subtable.

## Consequences

- Supplementary-plane mappings compose with the existing Character Map without a second key type.
- Exact length validation prevents trailing declared bytes from becoming an ambiguous extension.
- Prevalidating each group's last glyph bounds eager expansion and prevents arithmetic wraparound.
- Glyph IDs remain type-safe; validation against a face's actual glyph count remains a later table
  integration responsibility.
- Format 4 and format 12 can later be merged by one explicit encoding-record precedence policy.
