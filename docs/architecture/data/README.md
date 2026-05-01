# Data Architecture

Use this subtree for canonical storage and identifier rules.

## Read This Section When

- You need the PostgreSQL table layout.
- You need the canonical rules for opaque IDs or aliases.
- You need search indexing or consistency guarantees.

## Child Index

- [postgres-schema.md](postgres-schema.md): tables, columns, and required indexes
- [external-embed-cache.md](external-embed-cache.md): cached external URL metadata schema and refresh rules
- [id-rules.md](id-rules.md): opaque note and snapshot identifier rules
- [alias-rules.md](alias-rules.md): human-readable URL alias rules
- [search-indexing.md](search-indexing.md): derived search document fields and visibility rules
- [consistency-rules.md](consistency-rules.md): resource, saved-snapshot, navigation, and settings consistency rules
