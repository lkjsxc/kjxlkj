# Visual Verify Licenses

## Direct Components

| Component | Version | License Signal | How Repo Uses It |
|---|---:|---|---|
| `mcr.microsoft.com/playwright:v1.54.2-noble` | pinned image | image aggregate; includes browser binaries and system dependencies | Base image for `visual-verify` |
| `playwright` npm package | 1.54.2 | `Apache-2.0` | Installed explicitly in `Dockerfile.visual` |
| Chromium bundled in the Playwright image | image-provided | vendor browser terms and credits | Browser runtime for screenshots and interaction checks |
| Node.js runtime from the Playwright image | image-provided | image/base-image aggregate terms | Executes authored `src/verify/browser/*.mjs` scripts |

## Repo-Specific Clarifications

- `visual-verify` uses authored browser scripts plus Node built-ins and the `playwright` package.
- The repo does not add extra third-party npm packages beyond `playwright`.
- Browser binaries are not vendored into the source tree. They are inherited from the pinned Playwright image.
- Browser-license obligations therefore follow the image and the bundled browser vendors rather than the repo root `LICENSE`.

## Rules

- Keep the Playwright npm version pinned and aligned with the pinned Playwright image family.
- Treat the browser layer as a bundled third-party runtime, not as project-owned code.
- If `visual-verify` adds new npm dependencies, document them here before merging.
