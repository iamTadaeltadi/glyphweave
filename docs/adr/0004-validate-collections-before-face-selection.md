# ADR 0004: Validate font collections before face selection

- Status: Accepted
- Date: 2026-07-26

## Context

An OpenType Font Collection gives each Face Index an absolute offset to a complete Table
Directory. Faces may share whole tables, so validating each directory independently cannot detect
partial cross-face overlaps or a table that covers collection metadata. Version 2.0 headers can
also locate one file-level DSIG table. Callers need the same typed Face result for standalone fonts
and collections without managing raw offsets themselves.

## Decision

Represent a Face Index as a distinct unsigned domain type. A Face owns its validated Table
Directory and records the absolute directory offset used to decode it. `Face::select` accepts
immutable font bytes and a Face Index: standalone SFNT input accepts only index zero, while `ttcf`
input is decoded as a Font Collection before the requested Face is returned.

Font Collection decoding is eager. It accepts only header versions 1.0 and 2.0, rejects an empty
face list, validates every absolute directory offset, and decodes every Face before selection.
Header order defines Face Index order and is preserved even when directory offsets are not sorted.
Duplicate directory offsets are rejected because each advertised font resource must have its own
complete directory.

Collection-wide span validation permits tables to be shared only as the exact same byte range.
Partial cross-face table overlap and any overlap between tables, face directories, the collection
header, or a file-level signature are failures. A version 2.0 DSIG descriptor is either entirely
zero or uses the `DSIG` tag, identifies an in-bounds four-byte-aligned range, and ends at the end of
the file. Glyphweave validates that descriptor but does not authenticate the signature payload.

## Consequences

- Selecting one Face proves the container structure and all advertised directories are bounded.
- Shared tables retain zero-copy byte spans without being duplicated in memory.
- Malformed unselected faces still reject the collection instead of remaining latent hazards.
- DSIG authenticity remains outside the pure subsetting core; rewriting a collection will later
  drop or deliberately replace invalidated signatures.
