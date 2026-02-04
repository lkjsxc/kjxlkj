//! Command-line argument parsing.

/// Command-line arguments.
pub struct Args {
    /// File to open.
    pub file: Option<String>,
    /// Run in headless mode.
    pub headless: bool,
    /// Script file for headless mode.
    pub script: Option<String>,
}

impl Args {
    /// Parse command-line arguments.
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut file = None;
        let mut headless = false;
        let mut script = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--headless" => headless = true,
                "--script" => {
                    i += 1;
                    if i < args.len() {
                        script = Some(args[i].clone());
                    }
                }
                arg if !arg.starts_with('-') => {
                    if file.is_none() {
                        file = Some(arg.to_string());
                    }
                }
                _ => {}
            }
            i += 1;
        }

        Self {
            file,
            headless,
            script,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn args_module_compiles() {
        // Just verify the module compiles
        assert!(true);
    }
}
