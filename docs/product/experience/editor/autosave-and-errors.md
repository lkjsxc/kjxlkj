# Autosave and Errors Contract

## Autosave

- Note edits autosave after a short idle delay.
- Visibility changes use the same save pipeline as content changes.
- Initial page load does not trigger a no-op save.
- Identical body/privacy state does not trigger a new save request.
- Superseded saves may be ignored, but stale save results may not overwrite newer editor state.

## State Treatment

- Successful autosave is silent.
- `Saving` and `Saved` copy do not appear in normal UI.
- Save failure is shown with subdued but persistent error text until the next successful save.

## Live Sync

- The first `# ` heading keeps driving the page title, shell title, and browser title.
- Visibility changes update note chrome immediately.
