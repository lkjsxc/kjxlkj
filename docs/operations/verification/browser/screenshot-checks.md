# Screenshot Checks Contract

## Verification Command

```bash
docker compose --profile verify run --rm visual-verify
```

## Required Captures

- Desktop public root shell with rail and search.
- Desktop admin dashboard shell with rail and search.
- Desktop admin note page with rich editor.
- Desktop guest note page.
- Compact public root shell with stacked rail.
- Compact admin note page with stacked rail.

## Required Assertions

- Text-only actions remain readable and non-flashy.
- Visible timestamps are browser-local 24-hour strings.
- Raw note IDs are not shown in normal UI.
- No drawer controls or floating menu chrome appear.
- Surfaces remain flat with solid fills rather than gradients or blur.
- Note/history rails keep metadata readable without wrap regressions.
