# Privacy Model

## Rule

A Markdown file is private when frontmatter contains:

```yaml
private: true
```

## Enforcement

- Public list endpoints must filter private articles.
- Public article routes must deny private pages to logged-out users.
- CLI listing commands include visibility metadata.
- Admin-only endpoints always require a valid session.

## Threat Model Notes

- Private filtering happens before template rendering.
- Missing frontmatter defaults to non-private.
- Session cookies are HTTP-only and signed by server settings.
