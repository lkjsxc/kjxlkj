# Typed Metadata Records

Back: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)

## Model

Metadata is a per-note key-value map with typed JSON values.

| Field | Rule |
|---|---|
| `key` | lowercase slug, max 64 chars |
| `value` | valid JSON scalar/object/array |
| `updated_at` | UTC timestamp |

## API Behavior

- `PUT /notes/{id}/metadata/{key}` upserts value.
- `DELETE /notes/{id}/metadata/{key}` removes key.
- Projection reads MUST include current metadata map.

## Validation Rules

- Reserved prefixes (`system.`) are server-controlled.
- Metadata payload MUST stay under configured max size.

## Related

- Notes: [notes.md](notes.md)
- Types: [/docs/spec/api/types.md](/docs/spec/api/types.md)
