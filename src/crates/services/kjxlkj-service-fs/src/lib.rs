//! Filesystem service: async file IO.

/// Read a file's contents asynchronously.
pub async fn read_file(
    path: &std::path::Path,
) -> std::io::Result<String> {
    tokio::fs::read_to_string(path).await
}

/// Write contents to a file asynchronously.
pub async fn write_file(
    path: &std::path::Path,
    content: &str,
) -> std::io::Result<()> {
    tokio::fs::write(path, content).await
}
