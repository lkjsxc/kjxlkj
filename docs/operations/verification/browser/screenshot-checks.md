# Screenshot Checks Contract

## Verification Command

```bash
docker compose --profile verify run --rm visual-verify
```

## Required Captures

- Desktop homepage shell with rail.
- Desktop search page.
- Desktop admin dashboard shell.
- Desktop admin settings page.
- Desktop admin note page with the Markdown editor.
- Desktop guest note page.
- Desktop history index page showing current note plus revisions.
- Compact homepage shell closed by default.
- Compact homepage shell opened through the menu toggle.
- Compact admin note page with the drawer available.
- Compact admin note page with the preview overlay opened.

## Required Assertions

- Text-first actions remain readable and non-flashy.
- Desktop homepage spacing remains as restrained as compact homepage spacing.
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
- Search and history paging expose `Previous` and `Next` actions.
- Search sort and search-submit controls align vertically on desktop.
- Compact admin note pages do not overflow horizontally.
- Admin rails keep `New note` near the top.
- Rail navigation controls use the same size family as rail actions.
- Search sort remains functional without a visible `Sort` label.
- Rail brand spacing clearly separates `kjxlkj` from the first navigation card.
- The shell brand renders the circular icon cleanly on desktop and compact headers.
- HTML links `/favicon.ico` and the generated icon remains centered and legible at favicon scale.
- Opening an admin note should leave focus inside the visible editor.
- Preview starts closed by default.
- Compact preview opens as a fixed overlay.
- Compact preview stays correctly placed when the rail is drawer-only or closed.
- Newly typed Markdown renders correctly in the opened preview and in guest view after reload.
- Browser verification types into the visible Markdown editor surface rather than using `setMarkdown()`.
- Browser verification covers heading, list, blockquote, fenced code, and table authoring behavior.
- The editor toolbar wraps cleanly and does not expose a detached scrollbar strip.
- The main note page owns vertical scrolling; the editor body does not expose a second normal vertical scrollbar.
- Empty-query `/search` renders note cards instead of helper guidance.
- Search exposes a query display card near sort only when `q` is non-empty.
- Empty-query `/search` does not render a `Query` or `All notes` state card.
- Dashboard does not render a library block.
- Dashboard stacks `Settings`, `Popular notes`, `Recently updated`, and `Favorites`.
- Dashboard favorites follow persistent favorite order and expose reorder controls.
- Dashboard links to a dedicated settings page instead of embedding the full settings form.
- Settings page covers home title, intro Markdown, section visibility and order, section counts, search page size, and default new-note visibility.
- Dashboard and admin note surfaces expose note-view analytics.
- Markdown links are visibly accented in guest content and preview.
- Long URLs, code spans, and prose content do not trigger page-level horizontal overflow.
- Homepage intro Markdown renders when configured.
- Homepage popular-window switching updates the visible popular list.
- Homepage popular cards expose rolling-window and all-time totals.
- Homepage note sections expose `View more notes` cards for popular, recent, and favorites.
