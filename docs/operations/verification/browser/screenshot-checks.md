# Screenshot Checks Contract

## Verification Command

```bash
docker compose --profile verify run --rm visual-verify
```

## Required Captures

- Desktop public root shell with rail.
- Desktop search page.
- Desktop admin dashboard shell.
- Desktop admin note page with the single-mode editor.
- Desktop guest note page.
- Compact public root shell closed by default.
- Compact public root shell opened through the menu toggle.
- Compact admin note page with the drawer available.

## Required Assertions

- Text-first actions remain readable and non-flashy.
- Visible timestamps are browser-local 24-hour strings.
- Raw note IDs are not shown in normal UI.
- Rail search input does not appear.
- `RECENT` does not appear.
- `Rich mode`, `Text mode`, `Saving`, and `Saved` do not appear.
- Drawer toggle appears only on narrow screens.
- Narrow screens start with the drawer closed.
- Surfaces remain flat with solid fills rather than gradients or blur.
- Note and history rails keep metadata readable without wrap regressions.
