# ADR 0012: Resolve Loca into checked Glyf spans

- Status: Accepted
- Date: 2026-07-26

## Context

The `loca` table has no header. Its entry width comes from `head.indexToLocFormat`, its entry count
is `maxp.numGlyphs + 1`, and every value is an offset relative to the start of the `glyf` table.
Short entries store half-offsets while long entries store byte offsets. Consecutive values delimit
one Glyph ID's data; equal values represent a glyph without outline data.

Passing raw offsets downstream would make every outline decoder repeat arithmetic against three
different tables. It would also allow a caller to pair an otherwise valid `loca` table with the
wrong `glyf` span or glyph count. The normative structure is defined by the OpenType 1.9.1
[`loca`](https://learn.microsoft.com/en-us/typography/opentype/spec/loca),
[`head`](https://learn.microsoft.com/en-us/typography/opentype/spec/head),
[`maxp`](https://learn.microsoft.com/en-us/typography/opentype/spec/maxp), and
[`glyf`](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf) specifications.

## Decision

Define `LocaFormat` in `glyphweave-sfnt` as the closed `head.indexToLocFormat` domain: zero is
`Short` and one is `Long`. Keep the type at the dependency root so later `head` decoding and the
outline crate share one representation.

Define `Loca` in `glyphweave-outline`. Decode it from complete immutable font bytes, checked `loca`
and `glyf` Byte Spans, a Glyph count, and a `LocaFormat`. Require the `loca` span to contain exactly
`numGlyphs + 1` entries of the selected width. Expand short values by two with checked integer
semantics and preserve long values exactly.

Require offsets to start at zero, remain nondecreasing, be word aligned, and stay within the
unpadded `glyf` length. Require the final sentinel offset to equal that length. Equal consecutive
offsets are valid and produce empty glyph spans. Convert every valid pair into an absolute checked
Byte Span inside `glyf`; lookup by an out-of-range Glyph ID returns `None`.

Keep outline-specific failure values in a typed `OutlineCause` nested under the shared `Cause`.
This lets outline decoding construct the repository-wide span-aware `Diagnostic` without reversing
the frozen crate dependency graph.

Do not inspect glyph headers or require a minimum non-empty glyph length in this item. Simple and
composite Glyph decoding owns those content rules in the following progress items.

## Consequences

- Downstream outline code receives absolute bounded spans instead of repeating relative arithmetic.
- Short and long encodings resolve to the same canonical representation.
- Truncated, overlong, decreasing, odd, leading-gap, and trailing-gap location tables fail at the
  responsible table or entry span.
- Space glyphs and other glyphs without outlines retain a first-class empty span.
- A later emitter can choose an encoding from canonical offsets without coupling decoding to
  rewrite policy.
