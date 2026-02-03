#[derive(Clone, Debug)]
pub enum ServiceResult {
    FsReadOk {
        request_id: u64,
        path: String,
        contents: String,
    },
    FsWriteOk { request_id: u64, path: String },
    FsError { request_id: u64, message: String },
    TerminalOk { request_id: u64, output: String },
    TerminalError { request_id: u64, message: String },
}

