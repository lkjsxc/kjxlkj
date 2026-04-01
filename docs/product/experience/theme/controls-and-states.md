# Controls and States Contract

## Action Style

- Standard actions render as text-first controls.
- Filled buttons are not the default action language.
- Delete may use color emphasis, but still reads as text-first.
- Primary rail navigation controls use the same size family as `New note` and `Logout`.

## Compact Navigation Controls

- Narrow layouts expose one quiet top-right menu toggle.
- The drawer may use a backdrop but no loud close chrome.
- Rail interactions must remain keyboard accessible.
- Search and history paging controls should make backward and forward movement obvious.

## State Treatments

- Hover, focus, and active states are visible on every control.
- Disabled navigation stays readable and muted.
- Disabled timeline cards keep the same size as active cards.
- Save failure is explicit.
- Save success is silent.

## Form Rules

- `Public` remains the only visibility control.
- No helper sentence appears next to the checkbox.
- The rail must not expose a search box.
- The editor must not expose a visible mode switch.
- The product must not expose Vim-mode toggles, preferences, or status labels.
- Toolbar overflow resolves by wrapping, not by a separate scrollbar strip.
- Login page shows the brand only above the sign-in form and button.
- Editor metadata controls should share one card language rather than mixing unrelated field styles.
- Search query state may be displayed beside sort only when `q` is non-empty.
- Search sort keeps an accessible label through semantics rather than a visible helper word.
- Search input, sort control, and submit button align on one clean row on wide screens.
- Settings controls for section visibility, order, and limits stay aligned enough to scan as one system.

## Section Chrome

- Notes may keep bordered cards.
- Non-note sections such as `Quick search`, `Recently updated`, `Favorites`, and `Settings` should prefer spacing over boxed shells.
- Visible rules directly under section headings are not part of the product language.
- Section groups may separate through spacing, alignment, and shared background rather than repeated nested boxes.
