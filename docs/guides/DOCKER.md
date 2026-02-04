# Docker

Back: [/docs/guides/README.md](/docs/guides/README.md)
Running kjxlkj using Docker.

Container-based build/run support is required by policy (see `/docs/policy/WORKFLOW.md`).

`Dockerfile` and `.dockerignore` are derived artifacts in the “All in Docs” model: they may be absent in a docs-only baseline or after tooling cleanup, and MUST be regenerated when producing a shippable repository state.

## Build an image

- Build: `docker build -t kjxlkj:dev .`

## Run the editor (interactive TUI)

kjxlkj is a terminal UI app. Run with an interactive TTY:

- Run: `docker run --rm -it kjxlkj:dev`

## Run in headless mode

The binary supports `--headless` for non-interactive execution (useful in CI-like environments):

- Run: `docker run --rm kjxlkj:dev --headless`

## Working with files

To edit files from your host, mount a directory and pass a path:

- Example: `docker run --rm -it -v "$PWD":/work -w /work kjxlkj:dev path/to/file.txt`
