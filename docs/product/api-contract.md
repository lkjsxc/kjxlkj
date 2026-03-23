# API Contract

## Resource

The system manages `record` resources.

## Record Shape

```json
{
  "id": "alpha-note",
  "title": "Alpha Note",
  "body": "markdown or plain text",
  "tags": ["ops", "draft"],
  "revision": 3,
  "updated_at": "2026-03-23T00:00:00Z"
}
```

## Field Rules

- `id`: lowercase kebab-case.
- `title`: non-empty UTF-8 string.
- `body`: UTF-8 string, may be empty.
- `tags`: unique lowercase strings.
- `revision`: positive integer, increments on each successful write.
- `updated_at`: UTC RFC3339 timestamp.
