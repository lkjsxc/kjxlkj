# Privacy Policy Contract

## Private Marker

A Markdown article is private when frontmatter contains:

```yaml
private: true
```

## Enforcement Rules

- Public listings must omit private articles for logged-out users.
- Public article routes must deny private pages to logged-out users.
- Admin sessions may read and edit private articles.

## Default Behavior

- Missing frontmatter `private` is treated as `false`.
