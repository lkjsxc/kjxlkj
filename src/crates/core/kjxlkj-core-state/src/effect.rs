#[derive(Clone, Debug)]
pub enum Effect {
    Quit { force: bool },
    FsRead { request_id: u64, path: String },
    FsWrite {
        request_id: u64,
        path: String,
        contents: String,
    },
    TerminalRun { request_id: u64, command: String },
}

