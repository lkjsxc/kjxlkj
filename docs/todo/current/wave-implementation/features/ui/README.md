# Features: UI Features (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement the UI feature layer (cursor customization, viewport, popups, notifications, statusline).

## Defining documents (direct, normative)

- UI features index:
  - [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

## Coverage traversal

- UI features subtree:
  - [/docs/todo/doc-coverage/spec/features/ui/README.md](/docs/todo/doc-coverage/spec/features/ui/README.md)

## Checklist

- [x] Placeholder scaffolding: define feature state ownership and snapshot integration.
  - StatusLine with git_branch, git_status, diagnostics, lsp_status, file_type, encoding
  - DiagnosticSummary (errors/warnings/info/hints)
  - StatusComponent enum (16 variants)
  - NotificationLevel, Notification with timeout
  - PopupState, PopupContent (Text/Menu), PopupAnchor, MenuItem
  - TabEntry for buffer tabs
  - UiModel with tabs, notifications, popup fields
- [x] Minimal slice: implement one feature end-to-end with deterministic tests.
- [ ] Full conformance: implement all UI feature documents.
  - Cursor customization: CursorShape, CursorBlink, CursorLine, CursorConfig, CursorState
  - Notifications: NotificationKind, Notification, NotificationManager
  - Icons: FileType, GitStatus, DiagnosticLevel, DirectoryIcon, ActionIcon, ArrowIcon, IconConfig
  - Indent guides: IndentGuideStyle, IndentGuideConfig, IndentGuide, LineIndentGuides, ContextLine
  - Scroll customization: PastEnd, ScrollConfig, ScrollPosition with cursor-follow
  - Color picker: Color (RGB/HSL/hex), ColorMatch, ColorPicker, ColorPickerConfig
  - Statusline: StatusSection, StatusSegment, StatuslineConfig, TablineConfig, TabEntry
  - 288 tests total
- [x] Update conformance and limitations docs when user-visible. — done: conformance and limitations entries maintained with each batch

