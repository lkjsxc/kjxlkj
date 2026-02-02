//! Command-line (Ex) mode parsing.

use std::path::PathBuf;

/// A parsed command-line command.
#[derive(Debug, Clone)]
pub enum ExCommand {
    /// Write file (:w).
    Write { path: Option<PathBuf>, force: bool, all: bool },
    /// Quit (:q).
    Quit { force: bool, all: bool },
    /// Write and quit (:wq, :x).
    WriteQuit { path: Option<PathBuf>, force: bool, all: bool },
    /// Edit file (:e).
    Edit { path: PathBuf, force: bool },
    /// New buffer (:enew).
    NewBuffer,
    /// Buffer commands.
    Buffer(BufferCommand),
    /// Window commands.
    Window(WindowCommand),
    /// Tab commands.
    Tab(TabCommand),
    /// Set option (:set).
    Set { option: String, value: Option<String> },
    /// Substitute (:s).
    Substitute {
        pattern: String,
        replacement: String,
        flags: String,
        range: Option<String>,
    },
    /// Global command (:g).
    Global { pattern: String, command: String, inverse: bool },
    /// Normal command (:norm).
    Normal { keys: String, range: Option<String> },
    /// Help (:h).
    Help { topic: Option<String> },
    /// Version (:version).
    Version,
    /// Source file (:source).
    Source { path: PathBuf },
    /// Map key (:map, :nmap, etc.).
    Map { mode: String, lhs: String, rhs: String },
    /// Unmap key.
    Unmap { mode: String, lhs: String },
    /// Highlight (:hi).
    Highlight { group: String, settings: String },
    /// Colorscheme (:colo).
    Colorscheme { name: String },
    /// Command from registers (:@).
    RegisterCommand { register: char },
    /// Marks command (:marks).
    Marks { filter: Option<String> },
    /// Registers command (:reg).
    Registers { filter: Option<String> },
    /// Jumps command (:jumps).
    Jumps,
    /// Changes command (:changes).
    Changes,
    /// Command history (:history).
    History { kind: Option<String> },
    /// Make (:make).
    Make { args: String },
    /// Grep (:grep).
    Grep { pattern: String, path: Option<String> },
    /// Execute shell command (:!).
    Shell { command: String },
    /// Read file/command (:r).
    Read { source: String },
    /// Lua command (:lua).
    Lua { code: String },
    /// User-defined command.
    User { name: String, args: String },
    /// Unknown command.
    Unknown { line: String },
}

/// Buffer-related commands.
#[derive(Debug, Clone)]
pub enum BufferCommand {
    /// List buffers (:ls).
    List { all: bool },
    /// Go to buffer (:b).
    Go { target: BufferTarget },
    /// Delete buffer (:bd).
    Delete { target: BufferTarget, force: bool },
    /// Next buffer (:bn).
    Next,
    /// Previous buffer (:bp).
    Previous,
    /// First buffer (:bf).
    First,
    /// Last buffer (:bl).
    Last,
}

/// Target for buffer commands.
#[derive(Debug, Clone)]
pub enum BufferTarget {
    Current,
    Number(usize),
    Name(String),
    All,
}

/// Window-related commands.
#[derive(Debug, Clone)]
pub enum WindowCommand {
    /// Split horizontally (:sp).
    SplitHorizontal { path: Option<PathBuf> },
    /// Split vertically (:vs).
    SplitVertical { path: Option<PathBuf> },
    /// Close window (:close).
    Close { force: bool },
    /// Only this window (:only).
    Only,
    /// New window (:new).
    New,
    /// Vertical new (:vnew).
    VerticalNew,
}

/// Tab-related commands.
#[derive(Debug, Clone)]
pub enum TabCommand {
    /// New tab (:tabnew).
    New { path: Option<PathBuf> },
    /// Close tab (:tabclose).
    Close { force: bool },
    /// Next tab (:tabnext).
    Next,
    /// Previous tab (:tabprev).
    Previous,
    /// Go to tab (:tabn).
    Go { index: usize },
    /// Only this tab (:tabonly).
    Only,
}
