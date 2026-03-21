# Markdown Frontmatter Contract

## Support

Frontmatter is optional YAML between `---` delimiters.

Supported keys:

- `title: <string>`
- `private: <bool>`

## Example

```yaml
---
title: Welcome
private: false
---
# Hello
```

## Parsing Rules

- Missing `private` defaults to `false`.
- Invalid frontmatter must surface deterministic validation errors.
