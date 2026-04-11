# Partial Navigation Contract

## Scope

- Same-origin shell links may transition without a full browser reload.
- Mutating forms, external links, downloads, setup, login, password reset, and modified clicks use normal navigation.
- Failed partial fetches or parse errors fall back to normal browser navigation.

## Browser History

- Each successful shell transition owns exactly one browser-history entry.
- Browser back/forward must reopen the previously rendered shell page without adding a duplicate entry.
- If a guarded transition is canceled, the browser URL must revert to the current page instead of drifting away from the rendered shell state.
- Stale responses from superseded transitions must not win over the newest target URL.

## State Preservation

- The rail scroll position is preserved across successful partial transitions.
- Compact drawer open state is preserved when the target page also uses the shell.
- Browser history, document title, canonical links, robots metadata, and social-card metadata update to the target page.
- Local time formatting and page-specific scripts re-run after the shell swap.

## Navigation Guards

- Shell pages may register a leave guard that can cancel link clicks and browser back/forward.
- Dirty settings pages use that guard to prompt before leaving `/admin/settings`.

## Editor Guard

- Dirty note editors flush before a partial transition leaves the current resource.
- In-flight uploads or saves complete before the transition proceeds.
- If the flush fails, the transition is canceled and the editor shows the save error.
