# Root Shell Contract

## Global Rule

- Every HTML page renders inside the same shell structure.
- The side menu is always visible on root, admin, admin-settings, note, history, and search pages.
- Narrow screens switch the side menu into an overlay drawer.
- The brand area shows the product name only and carries no marketing caption.
- Desktop pages do not use a top-right action cluster for browse, search, or auth links.
- Rectangular shell cards and actions use tight corners instead of soft rounded shells.
- Same-origin shell transitions should preserve rail state instead of feeling like a full reload.

## Rail Content

- Brand and session mode.
- Global navigation.
- One external-repository section linking to the project GitHub page.
- Home, dashboard, and settings quick actions when appropriate.
- Current-note context when applicable.
- Timeline and history affordances when applicable.
- Mode-specific actions.
- Rail action ordering follows [actions/section-order.md](actions/section-order.md).
- Global navigation order is `Home`, `Search`, `Live`, then admin-only entries.
- Groups are separated visually, but visible section headings and heading underlines are omitted.

## Main Pane

- The non-rail main pane uses one shared maximum content width across root, admin, settings, search, resource, and history pages.
- Page headers, setting rows, resource metadata, resource navigation strips, prose surfaces, and editor surfaces align to that shared width.
- Individual media, prose, or editor elements may constrain their internal content, but their outer surface still aligns to the shared main width.
- Root, admin, admin-settings, and search pages keep home, browse, settings, stats, and query work in the main pane.
- Note and history pages keep document-focused content in the main pane.
- The rail adds navigation and context without repeating explanatory copy that the main pane already makes obvious.
- Partial navigation rules are owned by [partial-navigation.md](partial-navigation.md).
