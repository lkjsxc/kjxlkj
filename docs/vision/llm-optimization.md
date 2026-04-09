# LLM Optimization Rules

## Formatting Rules

- Use stable section names such as `Goal`, `Rules`, `Contract`, and `Verification`.
- Keep one requirement per bullet.
- Keep canonical definitions in one file and link outward.
- Prefer short declarative statements over narrative paragraphs.
- Delete obsolete contracts instead of preserving conflicting versions.

## Topology Rules

- Every docs directory has exactly one `README.md` TOC.
- TOCs link all immediate children.
- No directory may contain a single child artifact.

## Length Rules

- Docs files: `<= 300` lines.
- Authored source files: `<= 200` lines.

## Resource Rules

- `resource` is the shared term for live `note` and `media` records.
- Notes and media share IDs, aliases, visibility, favorites, history, and search participation.
- Notes store canonical Markdown in `body`.
- Media stores canonical Markdown in `body` plus one current binary object in S3-compatible storage.
- Markdown images render inline from standard image syntax.
- Markdown video embeds render from safe inline HTML `<video>` markup.
- Saved snapshots keep immutable binary object references for media.
- Initial installs default new resources to public unless settings change that default.
