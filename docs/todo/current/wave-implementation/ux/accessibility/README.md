# UX: Accessibility (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ux/README.md](/docs/todo/current/wave-implementation/ux/README.md)

## Scope

Implement accessibility requirements (keyboard-only, focus clarity, visibility constraints).

## Defining documents (direct, normative)

- Accessibility:
  - [/docs/spec/ux/accessibility.md](/docs/spec/ux/accessibility.md)

## Checklist

- [x] Turn accessibility requirements into testable acceptance criteria. — done: `accessibility.rs` with WCAG 2.1 contrast ratio, `A11yCheck`, `FocusIndicator`
- [x] Ensure cursor and focus are always visible and unambiguous. — done: `check_focus_visible()`, `check_color_scheme()`, `FocusIndicator` enum
- [x] Add tests for focus transitions and visibility across modes. — done: 7 tests covering contrast ratio, focus visibility, ARIA hints, color scheme checks

