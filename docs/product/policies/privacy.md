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
- Non-admin menu and search surfaces must omit private articles.
- Public article rendering must not display author attribution/byline metadata.

## Default Behavior

- Articles are private by default.
- Missing frontmatter `private` is treated as `true`.
