# ADR 0011: Preserve compressed Variation Selector mappings

- Status: Accepted
- Date: 2026-07-26

## Context

OpenType `cmap` format 14 describes Unicode variation sequences rather than standalone character
mappings. Each sequence combines a base scalar with one of the Unicode Variation Selectors. A
default UVS delegates to the ordinary Unicode Character Map, while a non-default UVS names its
Glyph ID directly. Therefore format 14 cannot be flattened into `CharacterMap`: doing so would
lose the selector, the default-delegation rule, and the distinction between an unsupported
sequence and a supported sequence that explicitly resolves to `.notdef`.

The format stores 24-bit values and relative offsets. Its selector records point to variable-sized
default-range and non-default-mapping tables, and multiple records can legally reuse identical
target bytes. Expanding every four-byte default range into as many as 256 map entries would impose
input-independent memory amplification.

The normative structure and ordering rules are defined by the OpenType 1.9.1 `cmap` specification:
<https://learn.microsoft.com/en-us/typography/opentype/spec/cmap>.

## Decision

Introduce a `VariationSelector` scalar type that can only contain U+FE00 through U+FE0F or U+E0100
through U+E01EF. Introduce `VariationMapping` with distinct `Default` and `NonDefault(GlyphId)`
cases. Preserve a non-default glyph-zero value instead of treating it as an absent sequence.

Store decoded mappings in `VariationSelectorMap`. Keep default mappings as ordered, disjoint
inclusive base-scalar ranges grouped by selector. Keep non-default mappings as ordered explicit
base-scalar to Glyph ID entries. Lookup returns the encoded mapping kind; resolution accepts a
`CharacterMap`, delegates default sequences to its base-scalar mapping, and returns `None` only for
an unsupported variation sequence. Expose compressed default ranges and explicit non-default
mappings separately for later deterministic subsetting and emission.

Define `CmapFormat14` in `glyphweave-sfnt`. Decode from complete immutable bytes and an absolute
subtable offset. Validate format 14, a declared length of at least ten bytes, complete availability,
and capacity for every eleven-byte Variation Selector record. Require selector records to be
strictly increasing and to contain typed Variation Selectors.

Treat zero target offsets as absent tables. For each nonzero relative offset, require a complete
four-byte count after the selector-record array and within the declared subtable. Derive the exact
target span with checked arithmetic: four bytes per default range and five bytes per non-default
mapping. Validate default ranges as sorted, disjoint Unicode-scalar ranges whose 24-bit start plus
eight-bit additional count does not overflow. Validate non-default base values as strictly ordered
Unicode scalars. Reject a base listed as both default and non-default for one selector.

Allow records to share a target only when its kind and complete Byte Span are identical. Reject
partial overlap, cross-kind aliasing, and targets that overlap the header or selector-record array.
Cache decoded targets by kind and relative offset so shared tables are parsed once. Unreferenced
bytes and gaps inside the declared subtable remain permitted because offset-based layouts need not
be packed.

Keep platform ID 0, encoding ID 5, outer `cmap` precedence, and validation against a face's actual
glyph count for the later table-level decoder. Format 14 itself validates only its own byte and
Unicode domains.

## Consequences

- Variation-sequence support remains distinguishable from its eventual Glyph ID, including
  supported sequences that resolve to `.notdef`.
- Default UVS storage stays proportional to encoded range count instead of expanded sequence count.
- The typed selector excludes ordinary scalars before they can become map keys.
- Checked target spans and overlap validation prevent records from reinterpreting header bytes or
  partially aliasing unrelated nested structures.
- Later rewriting can preserve semantics without recovering ranges from an eagerly expanded map.
