//! kjxlkj-host - Terminal host and event loop.
//!
//! This crate provides the terminal runtime that connects
//! the editor core to the terminal.

mod host;

pub use host::Host;

/// Run the editor with optional file path.
pub fn run(file: Option<std::path::PathBuf>) -> std::io::Result<()> {
    let mut host = Host::new()?;

    if let Some(path) = file {
        host.open_file(&path)?;
    }

    host.run()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_loads() {
        // Just verify the module compiles
        assert!(true);
    }
}
