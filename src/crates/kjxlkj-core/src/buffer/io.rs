use super::Buffer;

impl Buffer {
    /// Saves the buffer to its file path.
    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.path {
            std::fs::write(path, self.text())?;
            self.mark_saved();
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file path set",
            ))
        }
    }

    /// Saves the buffer to a specific path.
    pub fn save_as(&mut self, path: std::path::PathBuf) -> std::io::Result<()> {
        std::fs::write(&path, self.text())?;
        self.path = Some(path);
        self.mark_saved();
        Ok(())
    }
}
