# Screenshot Checks Contract

## Verification Command

```bash
docker compose --profile verify run --rm visual-verify
```

## Required Captures

- Desktop public root list with search.
- Desktop admin dashboard list with search.
- Desktop admin note page with plain editor.
- Desktop guest note page.
- Compact note page with drawer closed.
- Compact note page with drawer open.

## Required Assertions

- Text-only actions remain readable and non-flashy.
- Visible timestamps are browser-local 24-hour strings.
- Raw note IDs are not shown in normal UI.
- Compact menu and close controls stay visually quiet.
- Note/history rails keep metadata readable without wrap regressions.
