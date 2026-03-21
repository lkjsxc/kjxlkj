# kjxlkj Documentation

This directory is the canonical contract for the new `kjxlkj` project.

## Product Summary

`kjxlkj` is a single-admin Markdown publishing system.

- Logged-out users can browse public articles.
- Logged-in admin can edit Markdown files directly in a web UI.
- Each article can be marked private with frontmatter `private: true`.
- CLI commands are optimized for AI agent workflows.

## Directory Table of Contents

- [architecture/README.md](architecture/README.md)
- [product/README.md](product/README.md)
- [operations/README.md](operations/README.md)

## Global Constraints

- Every docs file must be 300 lines or fewer.
- Every source code file must be 200 lines or fewer.
- Every docs directory with multiple child docs/subdirectories must contain exactly one `README.md` as TOC.
- Docker Compose is the default verification entrypoint.
