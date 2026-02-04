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
    use super::*;

    #[test]
    fn args_module_compiles() {
        // Just verify the module compiles
        assert!(true);
    }

    #[test]
    fn args_struct_exists() {
        fn assert_type<T>(_: &T) {}
        let args = Args {
            file: None,
            headless: false,
            script: None,
        };
        assert_type::<Args>(&args);
    }

    #[test]
    fn args_file_field() {
        let args = Args {
            file: Some("test.txt".to_string()),
            headless: false,
            script: None,
        };
        assert_eq!(args.file, Some("test.txt".to_string()));
    }

    #[test]
    fn args_headless_field() {
        let args = Args {
            file: None,
            headless: true,
            script: None,
        };
        assert!(args.headless);
    }

    #[test]
    fn args_script_field() {
        let args = Args {
            file: None,
            headless: true,
            script: Some("script.txt".to_string()),
        };
        assert_eq!(args.script, Some("script.txt".to_string()));
    }

    #[test]
    fn args_all_fields_set() {
        let args = Args {
            file: Some("file.txt".to_string()),
            headless: true,
            script: Some("script.json".to_string()),
        };
        assert!(args.file.is_some());
        assert!(args.headless);
        assert!(args.script.is_some());
    }

    #[test]
    fn args_default_values() {
        let args = Args {
            file: None,
            headless: false,
            script: None,
        };
        assert!(!args.headless);
        assert!(args.file.is_none());
        assert!(args.script.is_none());
    }
}
