# UI: Popups and Overlays (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ui/README.md](/docs/todo/current/wave-implementation/ui/README.md)

## Scope

Implement popups/overlays (completion, picker, confirmations) and their interaction with cursor visibility.

## Defining documents (direct, normative)

- Popup API:
  - [/docs/spec/features/ui/popup-api.md](/docs/spec/features/ui/popup-api.md)

## Checklist

- [ ] Placeholder scaffolding: define overlay layering and focus rules.
- [ ] Minimal slice: implement one overlay with deterministic tests.
  - popup_menu.rs: PopupMenu with scroll/selection, HoverTooltip, compute_rect() with anchor positioning
- [ ] Full conformance: implement all overlay and popup behaviors.
  - popup_overlay.rs: PopupKind, PopupAnchor, PopupState (show/hide/select_next/prev/visible_items), OverlayManager, compute_popup_rect

