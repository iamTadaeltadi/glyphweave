# ADR 0017: Canonicalize layout glyph partitions before emission

- Status: Accepted
- Date: 2026-07-26

## Context

OpenType Coverage and Class Definition tables each have two wire formats. Coverage can list glyphs
or ranges while preserving a sequential Coverage Index. Class Definition can store a dense class
array or sorted ranges, with every unrepresented glyph implicitly assigned to class zero. The two
encodings create multiple byte representations for the same layout meaning.

Later GSUB and GPOS passes need fast membership, stable iteration, and deterministic remapping.
Retaining the source representation would make those passes preserve packing accidents and repeat
validation of ordering, overlap, and Coverage Index continuity.

## Decision

Decode Coverage into a strictly increasing owned sequence of unique `GlyphId` values. Decode Class
Definition into an ordered sparse map from `GlyphId` to nonzero class values; source class-zero
assignments are discarded because they are semantically identical to an omitted glyph. These
canonical models do not retain source spans. Every rejected field is instead reported immediately
with its exact absolute byte span.

Coverage format 1 must be strictly increasing. Coverage format 2 must contain valid, sorted,
non-overlapping ranges whose starting Coverage Indices form one uninterrupted sequence beginning
at zero. Class Definition format 1 must not extend beyond glyph ID 65535. Format 2 ranges must be
valid, sorted, and non-overlapping. Adjacent ranges are accepted because the specification does not
forbid them; canonicalization removes their representational significance.

Encode each canonical value using the shorter legal format. Coverage format 1 writes individual
glyphs and format 2 writes maximal consecutive ranges. Class Definition format 1 spans the first
through last nonzero assignment and fills holes with class zero; format 2 writes maximal
consecutive runs with the same nonzero class. Prefer format 1 on equal byte lengths. An empty
Class Definition therefore uses the four-byte empty format 2, while an empty Coverage uses the
four-byte format 1 tie winner.

Decoding and encoding remain pure in-memory operations in `glyphweave-layout`. Encoding a
Coverage containing all 65,536 glyph IDs must use format 2 because format 1's count is only 16
bits. All arithmetic is widened before range or size calculations.

## Consequences

- GSUB and GPOS code consumes one representation per semantic glyph partition.
- Binary search and ordered iteration are deterministic and independent of source packing.
- Malformed range metadata cannot survive into closure or rewriting.
- Canonical emission minimizes these structures without a global optimization pass.
- Exact source packing is intentionally not preserved.

## Source

OpenType 1.9.1, [Coverage and Class Definition tables](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#coverage-table).
