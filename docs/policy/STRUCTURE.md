# Documentation Structure Policy

This document defines mandatory structural constraints for all documentation.

## Directory Constraints

| Constraint | Value | Rationale |
|------------|-------|-----------|
| Max items per directory | 12 | Cognitive load limit; enables quick scanning |
| README.md per directory | Required | Navigation entry point for each topic |
| Max lines per file | 200 | Focused documents; split if exceeds |

## Directory Hierarchy Requirements

Every directory under `docs/` MUST contain exactly one `README.md`.

The README.md MUST:

- Introduce the directory's scope
- Link to all child documents and subdirectories
- Provide navigation back to parent

## File Count Enforcement

When a directory approaches or exceeds 12 items:

1. Identify logical groupings among the files
2. Create subdirectories for each group
3. Move files into appropriate subdirectories
4. Create README.md in each new subdirectory
5. Update parent README.md to link to new subdirectories

## Line Count Enforcement

When a file approaches or exceeds 200 lines:

1. Identify logical sections that can stand alone
2. Split into multiple files with focused topics
3. Create a parent file or README.md linking the split files
4. Ensure all cross-references are updated

## Navigation Requirements

| Requirement | Description |
|-------------|-------------|
| Reachability | Every document MUST be reachable via navigation from docs/README.md |
| Bidirectional links | Parent README.md links to children; children link to parent |
| No orphan documents | No document may exist without being linked |
| Link validation | All internal links MUST be verified on change |

## LLM-Focused Note Consolidation

The documentation intent statement MUST appear only in:

- [docs/README.md](docs/README.md)
- [docs/policy/README.md](README.md)

Leaf documents MUST NOT contain individual LLM-focused meta comments. Remove any such scattered notices during restructuring.

## Compliance Checklist

For any PR affecting documentation:

- [ ] No directory exceeds 12 direct children
- [ ] All directories contain exactly one README.md
- [ ] No file exceeds 200 lines
- [ ] All documents are navigable from docs/README.md
- [ ] No orphan documents exist
- [ ] LLM meta-notes appear only in designated locations
