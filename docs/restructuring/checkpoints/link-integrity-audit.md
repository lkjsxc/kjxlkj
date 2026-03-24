# Link Integrity Audit

## Objective

Ensure all relative markdown links in `docs/restructuring/**` resolve to existing files.

## Required Checks

- Link targets exist.
- No links point to deleted legacy restructuring paths.
- Parent TOCs include newly created child entries.

## Suggested Audit Command

```bash
python - <<'PY'
from pathlib import Path
import re
root = Path('docs/restructuring')
pattern = re.compile(r'\[[^\]]+\]\(([^)]+)\)')
for file in root.rglob('*.md'):
    text = file.read_text(encoding='utf-8')
    for target in pattern.findall(text):
        if '://' in target or target.startswith('#'):
            continue
        path = (file.parent / target.split('#')[0]).resolve()
        if not path.exists():
            raise SystemExit(f'broken link: {file} -> {target}')
print('link-audit: pass')
PY
```

## Output Rule

Record result in [../evidence/final.md](../evidence/final.md).
