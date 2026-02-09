use std::path::PathBuf;

/// Request from core to a service.
#[derive(Debug, Clone)]
pub enum ServiceRequest {
    /// Read file from disk.
    ReadFile { request_id: u64, path: PathBuf },
    /// Write file to disk.
    WriteFile {
        request_id: u64,
        path: PathBuf,
        content: String,
    },
    /// Git status query.
    GitStatus { request_id: u64 },
    /// Index workspace files.
    IndexWorkspace { request_id: u64, root: PathBuf },
}

/// Response from a service to core.
#[derive(Debug, Clone)]
pub enum ServiceResponse {
    /// File read result.
    FileRead {
        request_id: u64,
        content: Result<String, String>,
    },
    /// File write result.
    FileWritten {
        request_id: u64,
        result: Result<(), String>,
    },
    /// Git status result.
    GitStatusResult {
        request_id: u64,
        status: Result<String, String>,
    },
    /// Index result.
    IndexResult {
        request_id: u64,
        files: Result<Vec<PathBuf>, String>,
    },
}
