//! Action enum — the unified vocabulary for editor actions.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::action_sub::{CommandKind, InsertPosition};
use crate::{Direction, Motion, Operator, RegisterName, ScrollDirection, TextObject, VisualKind};

/// The unified action type dispatched from input to core.
///
/// Per /docs/spec/architecture/input-decoding.md, this enum covers
/// movement, editing, mode, command, buffer, window, search,
/// undo, macro, and system categories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Action {
    // Core editing / modes
    MoveCursor(Motion, u32), Scroll(ScrollDirection, u32), InsertChar(char), Delete(Motion, u32), Change(Motion, u32), Yank(Motion, u32),
    Put(bool), ReplaceChar(char), OperatorTextObject(Operator, TextObject, u32), DoubleOperator(Operator, u32), SubstituteChar, SubstituteLine,
    ChangeToEnd, DeleteCharForward, DeleteCharBackward, JoinLines, JoinLinesNoSpace, ToggleCaseChar, DotRepeat, Increment(i64),
    EnterInsert(InsertPosition), EnterVisual(VisualKind), EnterCommand(CommandKind), EnterReplace, ReturnToNormal, InsertNormal,
    EnterOperatorPending(Operator),

    // Command line
    ExecuteCommand(String), CmdlineChar(char), CmdlineBackspace, CmdlineComplete, CmdlineHistory(Direction),

    // Files, buffers, windows, tabs
    OpenFile(PathBuf), Write, WriteQuit, Quit, ForceQuit, QuitAll, WriteAll, WriteAllQuit, SwitchBuffer(String), NextBuffer, PrevBuffer,
    DeleteBuffer, AlternateFile, SplitHorizontal, SplitVertical, CloseWindow, FocusWindow(Direction), CycleWindow, ResizeWindow(Direction, i32),
    EqualizeWindows, MoveWindow(Direction), ZoomWindow, RotateWindows(bool), OnlyWindow, HideWindow, ExchangeWindow, FocusTopLeft,
    FocusBottomRight, FocusPrevWindow, MoveWindowToTab, NewSplit, NewVsplit, SplitOpen(String), VsplitOpen(String), ResizeCmd(String),
    TabNew(Option<String>), TabClose, TabOnly, TabNext, TabPrev, TabFirst, TabLast, TabGoto(usize), TabMove(String),

    // Search
    SearchForward(String), SearchBackward(String), NextMatch, PrevMatch,

    // Undo / marks / macros / registers
    Undo, Redo, SetMark(char), JumpToMark(char), JumpToMarkLine(char), RecordMacro(char), StopRecordMacro, PlayMacro(char, u32),
    SetRegister(RegisterName),

    // Terminal / system events
    Resize(u16, u16), Paste(String), FocusGained, FocusLost, QuitSignal, SpawnTerminal,

    // Sessions
    SessionSave, SessionLoad,

    // Ex commands and advanced actions
    Substitute(String), InsertRegister(char), GlobalCommand(String), VglobalCommand(String), SortLines(String), RangeDelete(String),
    RangeYank(String), RangeCopy(String), RangeMove(String), RangeNormal(String), ReadFile(String), ShellCommand(String), FilterLines(String),
    ExecuteExpr(String),

    // Mapping / config
    MapCommand(String, String), UnmapCommand(String, String), UserCommand(String), SourceFile(String), SetOption(String),

    // LSP
    LspHover, LspCodeAction, LspFormat, LspRename(String), LspSignatureHelp, LspReferences, LspDocumentSymbols, LspWorkspaceSymbols,
    LspCodeLens, LspInlayHints, LspCallHierarchy, LspTypeHierarchy,

    // Git
    GitSigns, GitDiff, GitBlame,

    // Misc features
    FlashJump, IncludeSearch(String), MultiCursorAdd, MultiCursorAll, MultiCursorSkip, SnippetExpand, SnippetNext, SnippetPrev,
    SpellToggle, SpellNext, SpellPrev, AutoSaveToggle, UndoTreeToggle, NotificationDismiss,

    // Sentinel
    Nop,
}
