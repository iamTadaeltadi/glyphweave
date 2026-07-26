# ADR 0018: Canonicalize layout lists without losing indexed identity

- Status: Accepted
- Date: 2026-07-26

## Context

OpenType ScriptList and FeatureList tables combine ordered records, relative offsets, and arrays of
indices. Script and language tags identify unique records and must be alphabetically ordered.
Feature records should also be tag-ordered, but duplicate feature tags are legal because different
language systems can reference different implementations of the same feature.

Language-system feature indices and Feature lookup indices may occur in arbitrary order. Their
order does not control shaping: selected lookups are applied in LookupList order. Preserving that
arbitrary source order would therefore create multiple representations of the same meaning.

## Decision

Represent scripts and named language systems as ordered maps keyed by `Tag`. Reject malformed,
duplicate, or decreasing script and language tags. Represent FeatureList as an indexed ordered
sequence and accept equal adjacent feature tags while rejecting decreasing tags, preserving the
record identity required by every feature index.

Represent a LanguageSystem's optional feature references and a Feature's lookup references as
sorted unique vectors. Preserve the required feature separately, interpreting `0xFFFF` as absent.
Reject a nonzero reserved `lookupOrderOffset`.

Decode every Offset16 relative to the structure specified by OpenType. The public decoder receives
an absolute containing `ByteSpan`; targets must be nonzero, remain within that span, and begin after
the complete record array that owns the offset. A nullable default-language offset is the only
zero target. Exact target aliasing is accepted because OpenType tables may share structures.
Diagnostics identify the offset field and retain absolute font coordinates.

Feature-parameter offsets are retained as unresolved checked absolute positions. Their payload
format depends on the feature tag and is deliberately outside M3.2; a later feature-specific pass
must decode or declare retention for the payload before emission.

## Consequences

- Script and language lookup is logarithmic and deterministic.
- Feature indices remain stable even when tags repeat.
- Semantically set-valued references have one canonical ordering.
- Relative targets cannot escape the containing layout table or reinterpret owner records.
- Feature parameters remain explicit work rather than being copied as untyped bytes.

## Source

OpenType 1.9.1, [ScriptList, language system, and FeatureList tables](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#scripts-and-languages).
