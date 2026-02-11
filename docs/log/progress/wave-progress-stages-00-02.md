# Wave Progress Archive: Stages 00–02

Back: [wave-progress.md](wave-progress.md)

## Stage 00: Foundation Ingestion

### Waves 000–007
- Status: COMPLETE
- Evidence: All boxes checked, committed

## Stage 01: Architecture Core

### Waves 008–015
- Status: COMPLETE
- Evidence: All boxes checked, committed as 043b0f78
- Multi-task runtime architecture implemented

## Stage 02: Editing and Modes

### Wave 016: Scope Freeze and Input Mapping
- Status: COMPLETE
- Committed: d5dfa1c1
- Evidence: 76 tests pass, all files ≤ 199 lines
- Key: ~40 motions, PendingState, g/z prefix, operator composition, file splits

### Wave 017: Requirement Extraction and Normalization
- Status: COMPLETE
- Committed: 032b0aec
- Evidence: 91 tests pass, all files ≤ 200 lines
- Key: 10 operators, RangeType/Inclusivity, g-prefix ops, D/Y/double-ops, case transforms

### Wave 018: State Model and Data Flow Design
- Status: COMPLETE
- Committed: 1aa214b9
- Evidence: 98 tests pass, all files ≤ 200 lines
- Key: RegisterStore, ForceModifier, count multiplication, Filter op, dot-repeat, register prefix

### Wave 019: Command and Route Wiring
- Status: COMPLETE
- Committed: 7306f5a9
- Evidence: 125 tests pass, all files ≤ 200 lines
- Key: Vim regex compiler, Ex command parser, search system, command-line input, regex crate

### Wave 020: Boundary and Error Semantics
- Status: COMPLETE
- Committed: d265278d
- Evidence: 132 tests pass, all files ≤ 200 lines
- Key: Blackhole register, clipboard stubs, \c/\C/\o/\O/\H, put p/P, register wiring, cursor clamp

### Wave 021: Unit and Integration Coverage
- Status: COMPLETE
- Committed: 4a33e9fe
- Evidence: 142 tests pass, all files ≤ 200 lines
- Key: Read-only registers, insert tracking, :registers, filename/cmdline/search register wiring

### Wave 022: Live E2E and Race Validation
- Status: COMPLETE
- Committed: 3b3c77a4
- Evidence: 167 tests pass, all files ≤ 200 lines
- Key: Star/hash search, :nohlsearch, hlsearch, match_count, word_at, search_util split

### Wave 023: Ledger Finalization
- Status: COMPLETE
- Committed: 25bcc66d
- Evidence: 173 tests pass, all files ≤ 200 lines
- Key: g*/g#, search history, ignorecase/smartcase, % forward scan, bracket_pair
