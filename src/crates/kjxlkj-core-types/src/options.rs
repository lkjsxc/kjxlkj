//! Buffer-local options implementation.
//!
//! Options that can be set per-buffer or globally.

/// Option scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionScope {
    /// Global option.
    Global,
    /// Buffer-local option.
    Buffer,
    /// Window-local option.
    Window,
}

/// A typed option value.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue {
    /// Boolean option.
    Bool(bool),
    /// Integer option.
    Int(i64),
    /// String option.
    String(String),
}

impl OptionValue {
    /// Returns the boolean value if this is a Bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            OptionValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the integer value if this is an Int.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            OptionValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the string value if this is a String.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            OptionValue::String(v) => Some(v),
            _ => None,
        }
    }
}

/// Buffer-local options.
#[derive(Debug, Clone, Default)]
pub struct BufferOptions {
    /// Tab stop size.
    pub tabstop: usize,
    /// Soft tab stop size.
    pub softtabstop: Option<usize>,
    /// Shift width for indentation.
    pub shiftwidth: usize,
    /// Use spaces instead of tabs.
    pub expandtab: bool,
    /// Auto-indent new lines.
    pub autoindent: bool,
    /// Smart indentation.
    pub smartindent: bool,
    /// File type.
    pub filetype: String,
    /// Text width for wrapping.
    pub textwidth: usize,
    /// File format (unix, dos, mac).
    pub fileformat: String,
    /// File encoding.
    pub fileencoding: String,
    /// Whether buffer is modifiable.
    pub modifiable: bool,
    /// Whether buffer is read-only.
    pub readonly: bool,
}

impl BufferOptions {
    /// Creates new buffer options with defaults.
    pub fn new() -> Self {
        Self {
            tabstop: 8,
            softtabstop: None,
            shiftwidth: 8,
            expandtab: false,
            autoindent: false,
            smartindent: false,
            filetype: String::new(),
            textwidth: 0,
            fileformat: "unix".to_string(),
            fileencoding: "utf-8".to_string(),
            modifiable: true,
            readonly: false,
        }
    }

    /// Gets an option by name.
    pub fn get(&self, name: &str) -> Option<OptionValue> {
        match name {
            "tabstop" | "ts" => Some(OptionValue::Int(self.tabstop as i64)),
            "shiftwidth" | "sw" => Some(OptionValue::Int(self.shiftwidth as i64)),
            "expandtab" | "et" => Some(OptionValue::Bool(self.expandtab)),
            "autoindent" | "ai" => Some(OptionValue::Bool(self.autoindent)),
            "smartindent" | "si" => Some(OptionValue::Bool(self.smartindent)),
            "filetype" | "ft" => Some(OptionValue::String(self.filetype.clone())),
            "textwidth" | "tw" => Some(OptionValue::Int(self.textwidth as i64)),
            "fileformat" | "ff" => Some(OptionValue::String(self.fileformat.clone())),
            "modifiable" | "ma" => Some(OptionValue::Bool(self.modifiable)),
            "readonly" | "ro" => Some(OptionValue::Bool(self.readonly)),
            _ => None,
        }
    }

    /// Sets an option by name.
    pub fn set(&mut self, name: &str, value: OptionValue) -> bool {
        match name {
            "tabstop" | "ts" => {
                if let Some(v) = value.as_int() {
                    self.tabstop = v.max(1) as usize;
                    return true;
                }
            }
            "shiftwidth" | "sw" => {
                if let Some(v) = value.as_int() {
                    self.shiftwidth = v.max(0) as usize;
                    return true;
                }
            }
            "expandtab" | "et" => {
                if let Some(v) = value.as_bool() {
                    self.expandtab = v;
                    return true;
                }
            }
            "autoindent" | "ai" => {
                if let Some(v) = value.as_bool() {
                    self.autoindent = v;
                    return true;
                }
            }
            "filetype" | "ft" => {
                if let Some(v) = value.as_string() {
                    self.filetype = v.to_string();
                    return true;
                }
            }
            "textwidth" | "tw" => {
                if let Some(v) = value.as_int() {
                    self.textwidth = v.max(0) as usize;
                    return true;
                }
            }
            "modifiable" | "ma" => {
                if let Some(v) = value.as_bool() {
                    self.modifiable = v;
                    return true;
                }
            }
            "readonly" | "ro" => {
                if let Some(v) = value.as_bool() {
                    self.readonly = v;
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    /// Returns the effective indent size (shiftwidth or tabstop).
    pub fn indent_size(&self) -> usize {
        if self.shiftwidth == 0 {
            self.tabstop
        } else {
            self.shiftwidth
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_options_defaults() {
        let opts = BufferOptions::new();
        assert_eq!(opts.tabstop, 8);
        assert!(!opts.expandtab);
        assert!(opts.modifiable);
    }

    #[test]
    fn test_buffer_options_get() {
        let opts = BufferOptions::new();
        assert_eq!(opts.get("tabstop").unwrap().as_int(), Some(8));
        assert_eq!(opts.get("ts").unwrap().as_int(), Some(8));
        assert_eq!(opts.get("et").unwrap().as_bool(), Some(false));
    }

    #[test]
    fn test_buffer_options_set() {
        let mut opts = BufferOptions::new();
        assert!(opts.set("tabstop", OptionValue::Int(4)));
        assert_eq!(opts.tabstop, 4);

        assert!(opts.set("et", OptionValue::Bool(true)));
        assert!(opts.expandtab);
    }

    #[test]
    fn test_buffer_options_set_invalid() {
        let mut opts = BufferOptions::new();
        // Wrong type
        assert!(!opts.set("tabstop", OptionValue::Bool(true)));
        // Unknown option
        assert!(!opts.set("unknown", OptionValue::Int(1)));
    }

    #[test]
    fn test_option_value_accessors() {
        let b = OptionValue::Bool(true);
        let i = OptionValue::Int(42);
        let s = OptionValue::String("test".to_string());

        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);

        assert_eq!(i.as_int(), Some(42));
        assert_eq!(i.as_string(), None);

        assert_eq!(s.as_string(), Some("test"));
        assert_eq!(s.as_bool(), None);
    }

    #[test]
    fn test_indent_size() {
        let mut opts = BufferOptions::new();
        opts.tabstop = 8;
        opts.shiftwidth = 4;
        assert_eq!(opts.indent_size(), 4);

        opts.shiftwidth = 0;
        assert_eq!(opts.indent_size(), 8);
    }
}
