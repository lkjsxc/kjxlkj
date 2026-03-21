# kjxlkj

`kjxlkj` is a single-admin Markdown publishing system built for AI-agent-driven workflows.

- Public users browse non-private Markdown pages.
- Admin logs in to edit Markdown files directly in-browser.
- `private: true` frontmatter hides content from logged-out users.
- Rust + Actix Web + PostgreSQL + HTMX architecture.

## Repository TOC

- [docs/README.md](docs/README.md)
- [content/README.md](content/README.md)

## Quick Start

```bash
cp .env.example .env
docker compose up --build
```

Open `http://localhost:8080`.
