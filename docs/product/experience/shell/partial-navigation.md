# Partial Navigation Contract

## Scope

- Same-origin shell links may transition without a full browser reload.
- Mutating forms, external links, downloads, setup, login, password reset, and modified clicks use normal navigation.
- Failed partial fetches or parse errors fall back to normal browser navigation.

## State Preservation

- The rail scroll position is preserved across successful partial transitions.
- Compact drawer open state is preserved when the target page also uses the shell.
- Browser history, document title, canonical links, and robots metadata update to the target page.
- Local time formatting and page-specific scripts re-run after the shell swap.

## Editor Guard

- Dirty note editors flush before a partial transition leaves the current resource.
- In-flight uploads or saves complete before the transition proceeds.
- If the flush fails, the transition is canceled and the editor shows the save error.
