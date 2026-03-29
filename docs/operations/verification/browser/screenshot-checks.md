# Screenshot Checks Contract

## Verification Command

```bash
docker compose --profile verify run --rm visual-verify
```

## Required Captures

- Desktop homepage shell with rail.
- Desktop search page.
- Desktop admin dashboard shell.
- Desktop admin note page with the Markdown editor.
- Desktop guest note page.
- Desktop history index page showing current note plus revisions.
- Compact homepage shell closed by default.
- Compact homepage shell opened through the menu toggle.
- Compact admin note page with the drawer available.
- Compact admin note page with the preview overlay opened.

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
- Note rails render one `All history` card and never render inline revision lists.
- Timeline cards keep a stable two-card layout, including disabled placeholders.
- Search and homepage cards keep consistent heights without row-stretch artifacts.
- Compact admin note pages do not overflow horizontally.
- Admin rails keep `New note` near the top.
- Rail brand spacing clearly separates `kjxlkj` from the first navigation card.
- Opening an admin note should leave focus inside the visible editor.
- Preview starts closed by default.
- Compact preview opens as a fixed overlay.
- Newly typed Markdown renders correctly in the opened preview and in guest view after reload.
- Browser verification types into the visible Markdown editor surface rather than using `setMarkdown()`.
- Browser verification covers heading, list, blockquote, fenced code, and table authoring behavior.
- The editor toolbar wraps cleanly and does not expose a detached scrollbar strip.
- The main note page owns vertical scrolling; the editor body does not expose a second normal vertical scrollbar.
- Empty-query `/search` renders note cards instead of helper guidance.
- Dashboard does not render a library block.
- Dashboard settings cover default Vim mode and browser-local Vim override behavior.
