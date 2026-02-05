# Features: Window (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement window-management features (splits, tabs, layouts) beyond the core window model.

## Defining documents (direct, normative)

- Window features index:
  - [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

## Coverage traversal

- Window features subtree:
  - [/docs/todo/doc-coverage/spec/features/window/README.md](/docs/todo/doc-coverage/spec/features/window/README.md)

## Checklist

- [ ] Placeholder scaffolding: define window feature state and UI invariants.
- [ ] Minimal slice: implement one window workflow end-to-end with tests.
- [ ] Full conformance: implement all window feature documents.
  - Floating windows: FloatBorder, FloatAnchor, FloatSize, FloatConfig, FloatingWindow, FloatBounds
  - Window commands (wincmd): WinCmd enum with navigation, resize, close, rotate, etc.
  - Window zoom: ZoomState with toggle and layout preservation
  - Layout presets: LayoutPreset (Single/HorizontalStack/VerticalStack/Grid/MainLeft/etc.)
  - Resize modes: ResizeMode enum
  - 305 tests total in core-ui crate
- [ ] Update conformance and limitations docs when user-visible.

