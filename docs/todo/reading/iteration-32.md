# Reading Log: Iteration 32 (2026-02-03)

Back: [/docs/todo/reading/README.md](/docs/todo/reading/README.md)

## Documents read in-depth (≥ 50)

### Policy and indices

1. `/README.md`
2. `/docs/README.md`
3. `/docs/policy/README.md`
4. `/docs/policy/INSTRUCT.md`
5. `/docs/policy/STRUCTURE.md`
6. `/docs/policy/WORKFLOW.md`
7. `/docs/todo/README.md`
8. `/docs/spec/README.md`
9. `/docs/spec/how-to-read.md`
10. `/docs/overview/README.md`
11. `/docs/overview/principles.md`
12. `/docs/overview/glossary.md`

### Architecture, core, and UI

13. `/docs/spec/architecture/README.md`
14. `/docs/spec/architecture/runtime.md`
15. `/docs/spec/architecture/crates.md`
16. `/docs/spec/architecture/plugins.md`
17. `/docs/spec/editor/README.md`
18. `/docs/spec/editor/buffers.md`
19. `/docs/spec/editor/windows.md`
20. `/docs/spec/ui/README.md`
21. `/docs/spec/ui/components.md`
22. `/docs/spec/ui/views.md`
23. `/docs/spec/ui/themes.md`

### Editing and UX

24. `/docs/spec/editing/README.md`
25. `/docs/spec/editing/cursor/README.md`
26. `/docs/spec/editing/motions/README.md`
27. `/docs/spec/editing/motions/scroll-motions.md`
28. `/docs/spec/modes/README.md`
29. `/docs/spec/ux/layout.md`
30. `/docs/spec/ux/accessibility.md`
31. `/docs/spec/ux/keybindings.md`
32. `/docs/spec/ux/keybindings/navigation.md`

### Features: UI and terminal

33. `/docs/spec/features/README.md`
34. `/docs/spec/features/ui/README.md`
35. `/docs/spec/features/ui/cursor-customization.md`
36. `/docs/spec/features/ui/scroll-customization.md`
37. `/docs/spec/features/ui/viewport.md`
38. `/docs/spec/features/ui/font-rendering.md`
39. `/docs/spec/features/ui/ligatures.md`
40. `/docs/spec/features/ui/icons.md`
41. `/docs/spec/features/ui/popup-api.md`
42. `/docs/spec/features/ui/notifications.md`
43. `/docs/spec/features/ui/statusline/README.md`
44. `/docs/spec/features/ui/statusline/statusline.md`
45. `/docs/spec/features/terminal/README.md`
46. `/docs/spec/features/terminal/terminal.md`

### Commands and testing/performance

47. `/docs/spec/commands/README.md`
48. `/docs/spec/commands/syntax.md`
49. `/docs/spec/technical/contracts.md`
50. `/docs/spec/technical/memory.md`
51. `/docs/spec/technical/profiling.md`
52. `/docs/spec/technical/testing.md`
53. `/docs/technical/testing/load.md`
54. `/docs/technical/testing/regression.md`
55. `/docs/technical/unicode.md`
56. `/docs/technical/bidi.md`

## Contradictions and gaps recorded

- Many cross-directory links use `docs/...` paths that are not repo-root absolute, creating broken navigation.
- Some docs assume an in-repo Rust implementation exists (`Cargo.toml`, `src/`) while the repository is “All in Docs”.
- Several UI behavior specs (`viewport.md`, `scroll-motions.md`, `cursor-customization.md`) contain placeholder sections that do not fully define required behavior.
- Some UI docs include mouse-related placeholders that contradict the keyboard-only invariant.
