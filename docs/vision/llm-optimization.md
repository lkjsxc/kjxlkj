# LLM Optimization Rules

## Formatting Rules

- Use stable section names: `Goal`, `Rules`, `Contract`, `Verification`.
- Keep one requirement per bullet.
- Keep canonical definitions in one file and link to them.
- Prefer short declarative statements over narrative paragraphs.
- Delete obsolete contracts instead of preserving conflicting versions.

## Topology Rules

- Every docs directory has exactly one `README.md` TOC.
- TOCs link all immediate children.
- No directory may contain a single child artifact.

## Length Rules

- Docs files: <= 300 lines.
- Source files: <= 200 lines.

## Note Content Rules

- Title is extracted from the first `# heading` line in the body.
- Stored content remains canonical Markdown even when the editor shows rendered content.
- New-note visibility comes from the current global default and initial installs default that setting to private.
- Homepage hero content is split into an editable plain-text title plus optional Markdown intro body.
