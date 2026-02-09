# TODO: UI and Rendering

Back: [/docs/todo/current/areas/README.md](/docs/todo/current/areas/README.md)

## Normative Sources

- [/docs/spec/ui/README.md](/docs/spec/ui/README.md)
- [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
- [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
- [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)
- [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [/docs/spec/features/ui/statusline/README.md](/docs/spec/features/ui/statusline/README.md)
- [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)

## Inventory

- [x] Extract all UI/rendering requirements into requirement matrix.

## Implementation

- [x] Implement snapshot-to-frame pipeline per render spec.
- [x] Implement viewport, wrapping, CJK boundary, and cursor display invariants.
- [x] Implement statusline, popup, notification, and theming surfaces.

## Verification

- [x] Add/refresh deterministic rendering and viewport boundary tests.
- [x] Record evidence in conformance and limitations ledgers.
