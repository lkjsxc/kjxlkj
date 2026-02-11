# Wave Progress: Stage 05 Early (Waves 040–042)

Archived from wave-progress.md for file-size compliance.

### Wave 040: Scope Freeze and Input Mapping
- Status: COMPLETE
- Committed: 81b889f5
- Evidence: 473 tests pass, all files ≤ 200 lines
- Key: VT100/xterm escape parser (13 states), CSI dispatch, SGR attributes,
  private modes, OSC title, Screen model (cell grid, cursor, scroll region),
  filetype detection (15 languages + shebang). 31 new tests.

### Wave 041: Requirement Extraction and Normalization
- Status: COMPLETE
- Committed: 611219bf
- Evidence: 493 tests pass, all files ≤ 200 lines
- Key: LSP lifecycle model (phase machine, capabilities, crash tracking),
  diagnostic model (DiagnosticStore, severity sort, navigation), theme/highlight
  model (HlGroup, Color, Style, default_dark). 20 new tests.

### Wave 042: State Model and Data Flow Design
- Status: COMPLETE
- Committed: 66317c56
- Evidence: 515 tests pass, all files ≤ 200 lines
- Key: Git sign state model (GitSignState, hunk tracking, sign_at, navigation),
  statusline data model (Segment enum, StatuslineData), message/notification model
  (MsgLevel, MessageStore). 22 new tests.
