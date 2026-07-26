# ADR 0020: Normalize supported GSUB before retention and remapping

- Status: Accepted
- Date: 2026-07-26

## Context

GSUB defines direct, ligature, contextual, chained-contextual, and reverse-chaining
substitutions. Several types have multiple wire formats, while extension Lookup type 7 is only a
transport wrapper. Glyph closure must not depend on those encoding choices. Retention and dense
glyph remapping also need to agree about which rule inputs, contexts, outputs, and LookupList
references survive.

Treating unrecognized effective types as opaque bytes is unsafe: retained substitutions could
refer to discarded glyphs. Retaining every decoded record is also insufficient because contextual
rules can invoke other lookups and substitution outputs can enable more substitutions.

## Decision

Support every effective GSUB lookup type: Single (1), Multiple (2), Alternate (3), Ligature (4),
Context (5), Chained Context (6), and Reverse Chaining Single (8). Support every format defined for
those types by OpenType 1.9.1. Lookup type 7 is already flattened before GSUB dispatch.

Decode wire formats into semantic rules while preserving rule, set, sequence, and LookupRecord
order where order affects shaping. Direct substitution formats normalize to explicit input/output
mappings. Context formats retain their exact-glyph, class, or Coverage predicates so implicit
class zero is not expanded against an unknown font glyph universe.

Compute closure as a monotone fixed point over the selected Glyph Set. Direct rules add every
possible output for a retained input. Ligatures add an output only when every component is
retained. Context predicates are feasible only when each predicate intersects the retained set;
their ordered LookupRecords make referenced lookups reachable. Cyclic lookup references terminate
because both reachable lookup indices and glyph membership only grow.

After closure, discard infeasible rules and empty subtables, then remap every surviving glyph
through the canonical dense Remap. A missing mapping is a Rewrite-stage failure rather than a
silently dropped reference. Preserve old LookupList indices in this milestone; M3.6 owns pruning,
index compaction, and repair of Feature and contextual LookupRecord indices.

## Consequences

- Closure and retention operate on GSUB meaning, not its packed representation.
- Contextual lookup cycles are bounded without native recursion.
- Remapped structures cannot retain an input-space Glyph ID accidentally.
- Canonical byte encoding can choose compact wire formats after indices are repaired.
- Full GSUB support is larger than direct-substitution-only decoding but leaves no hidden opaque
  glyph references in retained GSUB data.

## References

- OpenType 1.9.1, [Glyph Substitution table](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub).
