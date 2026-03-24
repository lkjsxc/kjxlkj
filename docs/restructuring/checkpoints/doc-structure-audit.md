# Documentation Structure Audit

## Objective

Enforce recursive TOC and child-entry rules for all `docs/restructuring/**` directories.

## Required Checks

- Exactly one `README.md` exists in each directory.
- Every directory has multiple child entries.
- No orphan markdown files are unreachable from a parent TOC.

## Suggested Audit Command

```bash
python - <<'PY'
from pathlib import Path
root = Path('docs/restructuring')
for d in sorted([p for p in root.rglob('*') if p.is_dir()]):
    children = [c for c in d.iterdir() if not c.name.startswith('.')]
    readmes = [c for c in children if c.name == 'README.md']
    assert len(readmes) == 1, f'{d}: README contract violation'
    assert len(children) >= 3 or d == root, f'{d}: needs multiple children'
print('structure-audit: pass')
PY
```

## Output Rule

Record result in [../evidence/final.md](../evidence/final.md).
