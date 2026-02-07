# UI: Themes (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ui/README.md](/docs/todo/current/wave-implementation/ui/README.md)

## Scope

Implement theme models and rendering integration (color, styles, icons where specified).

## Defining documents (direct, normative)

- UI themes:
  - [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
- UI feature theming rules:
  - [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

## Checklist

- [ ] Placeholder scaffolding: define theme data model and apply rules.
- [ ] Minimal slice: implement a small theme set and verify deterministically.
- [ ] Full conformance: implement all theme behavior, including user customization.
  - theme_full.rs: ColorIndex, Rgb (hex/from_hex/luminance/is_dark), ThemeColor (Named/Rgb/Index/Default), Face (builder pattern), index_to_rgb (256-color), resolve_color

