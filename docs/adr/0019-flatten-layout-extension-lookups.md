# ADR 0019: Flatten validated layout extension lookups

- Status: Accepted
- Date: 2026-07-27

## Context

GSUB and GPOS share an indexed LookupList and Lookup header, but assign different meanings and
limits to `lookupType`. Lookup order and subtable order are semantic. GSUB type 7 and GPOS type 9
are transport mechanisms: each wrapper names the real lookup type and reaches its payload through
an Offset32. They do not define independent shaping operations.

Lookup flags combine independent glyph-class filters with precedence rules for marks. Reserved
bits must be zero. A present mark-filtering-set bit also changes the physical Lookup header by
adding a `markFilteringSet` field, even when `IGNORE_MARKS` makes that declaration ineffective.

## Decision

Decode LookupList entries and subtable offsets as indexed ordered sequences. Permit exact target
aliasing while rejecting null targets, targets inside their owning offset arrays or headers, and
targets outside the containing layout span. Reject empty Lookup subtables because a Lookup defines
one or more subtables.

Represent the layout table family explicitly and validate lookup types against GSUB 1 through 8 or
GPOS 1 through 9. Decode GSUB type 7 and GPOS type 9 wrappers eagerly, require format 1, reject
recursive extension types, and require every wrapper in one Lookup to resolve to the same real
type. Canonical Lookups retain that real type and the ordered payload spans, not the transport
wrappers. Later GSUB and GPOS passes therefore receive one dispatch shape.

Parse the optional mark-filtering-set field whenever its raw flag is present, then canonicalize
mark filtering by specification precedence: `IGNORE_MARKS`, mark filtering set, mark attachment
class, then no mark filter. Preserve the other independent flag booleans. GDEF availability and
mark-set/class index validity are cross-table checks deferred to M3.6.

## Consequences

- Lookup and subtable indices remain stable and processing order is explicit.
- Extension and direct payloads share the same downstream decoders.
- Extension Offset32 arithmetic is checked without narrowing to the Offset16 domain.
- Redundant mark-filter declarations do not create multiple canonical meanings.
- Structural validation remains independent from GDEF reference repair.

## Sources

- OpenType 1.9.1, [LookupList, Lookup, and lookup flags](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#lookuplist-table).
- OpenType 1.9.1, [GSUB substitution extension](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookup-type-7-subtable-substitution-subtable-extension).
- OpenType 1.9.1, [GPOS positioning extension](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-9-subtable-positioning-subtable-extension).
