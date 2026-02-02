use super::{OptionMeta, OptionScope, OptionValue, Options};

pub(super) fn register_defaults(opts: &mut Options) {
    opts.register(OptionMeta {
        name: "tabstop",
        short: Some("ts"),
        description: "Number of spaces a tab counts for",
        default: OptionValue::Int(8),
        scope: OptionScope::Buffer,
    });
    opts.register(OptionMeta {
        name: "shiftwidth",
        short: Some("sw"),
        description: "Number of spaces for indentation",
        default: OptionValue::Int(8),
        scope: OptionScope::Buffer,
    });
    opts.register(OptionMeta {
        name: "expandtab",
        short: Some("et"),
        description: "Use spaces instead of tabs",
        default: OptionValue::Bool(false),
        scope: OptionScope::Buffer,
    });
    opts.register(OptionMeta {
        name: "number",
        short: Some("nu"),
        description: "Show line numbers",
        default: OptionValue::Bool(false),
        scope: OptionScope::Window,
    });
    opts.register(OptionMeta {
        name: "relativenumber",
        short: Some("rnu"),
        description: "Show relative line numbers",
        default: OptionValue::Bool(false),
        scope: OptionScope::Window,
    });
    opts.register(OptionMeta {
        name: "wrap",
        short: None,
        description: "Wrap long lines",
        default: OptionValue::Bool(true),
        scope: OptionScope::Window,
    });
    opts.register(OptionMeta {
        name: "ignorecase",
        short: Some("ic"),
        description: "Ignore case in search patterns",
        default: OptionValue::Bool(false),
        scope: OptionScope::Global,
    });
    opts.register(OptionMeta {
        name: "smartcase",
        short: Some("scs"),
        description: "Override ignorecase if pattern has uppercase",
        default: OptionValue::Bool(false),
        scope: OptionScope::Global,
    });
    opts.register(OptionMeta {
        name: "hlsearch",
        short: Some("hls"),
        description: "Highlight search matches",
        default: OptionValue::Bool(false),
        scope: OptionScope::Global,
    });
    opts.register(OptionMeta {
        name: "incsearch",
        short: Some("is"),
        description: "Incremental search",
        default: OptionValue::Bool(true),
        scope: OptionScope::Global,
    });
    opts.register(OptionMeta {
        name: "scrolloff",
        short: Some("so"),
        description: "Minimum lines above/below cursor",
        default: OptionValue::Int(0),
        scope: OptionScope::Global,
    });
    opts.register(OptionMeta {
        name: "autoindent",
        short: Some("ai"),
        description: "Copy indent from current line",
        default: OptionValue::Bool(true),
        scope: OptionScope::Buffer,
    });
    opts.register(OptionMeta {
        name: "clipboard",
        short: Some("cb"),
        description: "Clipboard integration",
        default: OptionValue::String(String::new()),
        scope: OptionScope::Global,
    });
    opts.register(OptionMeta {
        name: "mouse",
        short: None,
        description: "Mouse mode",
        default: OptionValue::String("a".to_string()),
        scope: OptionScope::Global,
    });
}
