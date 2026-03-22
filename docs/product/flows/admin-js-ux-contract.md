# Inline Editing UX Contract

## Progressive Enhancement

- Inline editing works with plain form submission.
- JavaScript enhancement adds autosave timing and unload protection.
- Behavior stays deterministic with or without JavaScript.

## Autosave Contract

- Dirty state toggles on `title`, `private`, and `body` edits.
- Autosave triggers after 2 seconds idle.
- Blur triggers immediate save when dirty.
- Before unload triggers save attempt and unload warning when dirty.

## Accessibility Contract

- Inline status region uses `aria-live="polite"`.
- History and navigation links remain keyboard reachable.
- Editing controls appear only for authenticated admin.

## Removed Behaviors

- No save button.
- No preview button.
- No split-pane preview surface.
