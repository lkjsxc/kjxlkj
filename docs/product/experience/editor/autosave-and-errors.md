# Autosave and Errors Contract

## Autosave

- Note edits autosave after a short idle delay.
- Visibility, alias, and favorite changes use the same save pipeline as content changes.
- Initial page load does not trigger a no-op save.
- Identical body/privacy/alias/favorite state does not trigger a new save request.
- Superseded saves may be ignored, but stale save results may not overwrite newer editor state.
- Save responses may update the saved baseline only when they match the active draft generation.
- Autosave must not rewrite the textarea while the user is composing text or while a newer local edit exists.

## State Treatment

- Successful autosave is silent.
- `Saving` and `Saved` copy do not appear in normal UI.
- Save failure is shown with subdued but persistent error text until the next successful save.
- Successful media upload may show brief status inside the editor surface.
- If media upload falls back from a stale selection to append-at-end insertion, the status is `Selection changed; inserted at end.`
- Selection fallback status is informational and must not use error styling.
- Upload insertion uses the selection captured before the file picker opens.
- Upload insertion restores the caret from the server-confirmed updated body rather than from client-side prediction.
- Uploads wait for any in-flight save to settle before submitting the draft that should receive embeds.

## Navigation Guard

- Same-origin note transitions flush dirty editor state before leaving the current resource.
- A failed flush cancels the transition and keeps the user on the current editor.
- Uploads in progress block note transitions until the upload finishes or fails.

## Live Sync

- The first `# ` heading keeps driving the page title, shell title, and browser title.
- Visibility changes update note chrome immediately.
