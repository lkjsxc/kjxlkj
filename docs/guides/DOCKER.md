# Docker

Back: [/docs/guides/README.md](/docs/guides/README.md)

Container workflow for reconstructed repository states that include Docker artifacts.

## Scope

`Dockerfile` and `.dockerignore` are derived artifacts.

- They may be absent in docs-only baseline states.
- When present, this guide describes expected usage.

Root artifact expectations are defined in:

- [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)

## Build an image

- Build: `docker build -t kjxlkj:dev .`

## Run interactive TUI

- Run: `docker run --rm -it kjxlkj:dev`

## Run with host files

- Example: `docker run --rm -it -v "$PWD":/work -w /work kjxlkj:dev path/to/file.txt`

## Notes

- If a headless mode is implemented, its behavior must be tracked in conformance.
- If Docker artifacts are missing, reconstruct via `/docs/todo/` workflow first.

Status ledgers:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
