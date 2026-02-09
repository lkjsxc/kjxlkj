//! Session save/load.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Serializable session data.
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
    /// Open buffer paths.
    pub buffers: Vec<PathBuf>,
    /// Window layout description.
    pub layout: SessionLayout,
    /// Active buffer index.
    pub active: usize,
    /// Working directory.
    pub cwd: PathBuf,
}

/// Simplified layout for session persistence.
#[derive(Debug, Serialize, Deserialize)]
pub enum SessionLayout {
    Single,
    HorizontalSplit(Vec<SessionLayout>),
    VerticalSplit(Vec<SessionLayout>),
}

/// Save session data to a JSON file.
pub fn save_session(
    path: &std::path::Path,
    data: &SessionData,
) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("serialize: {e}"))?;
    std::fs::write(path, json)
        .map_err(|e| format!("write: {e}"))?;
    Ok(())
}

/// Load session data from a JSON file.
pub fn load_session(
    path: &std::path::Path,
) -> Result<SessionData, String> {
    let json = std::fs::read_to_string(path)
        .map_err(|e| format!("read: {e}"))?;
    let data: SessionData = serde_json::from_str(&json)
        .map_err(|e| format!("deserialize: {e}"))?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_session() {
        let data = SessionData {
            buffers: vec![PathBuf::from("test.txt")],
            layout: SessionLayout::Single,
            active: 0,
            cwd: PathBuf::from("/tmp"),
        };
        let json =
            serde_json::to_string_pretty(&data).unwrap();
        let loaded: SessionData =
            serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.buffers.len(), 1);
        assert_eq!(loaded.active, 0);
    }
}
