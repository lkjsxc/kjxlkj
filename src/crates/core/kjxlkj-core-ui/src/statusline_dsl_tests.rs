use super::*;

#[test]
fn parse_simple_format() {
    let tokens = parse_format("%f %m");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], DslToken::FilePath);
    assert_eq!(tokens[1], DslToken::Literal(" ".into()));
    assert_eq!(tokens[2], DslToken::Modified);
}

#[test]
fn parse_separator() {
    let tokens = parse_format("%f%=%l:%c");
    assert!(tokens.contains(&DslToken::Separator));
}

#[test]
fn parse_escaped_percent() {
    let tokens = parse_format("%p%%");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], DslToken::Percent);
    assert_eq!(tokens[1], DslToken::Literal("%".into()));
}

#[test]
fn parse_highlight_group() {
    let tokens = parse_format("%#StatusLine# text");
    assert_eq!(
        tokens[0],
        DslToken::Highlight("StatusLine".into())
    );
}

#[test]
fn render_basic() {
    let tokens = parse_format("%f %m %= %l:%c");
    let vars = DslVars {
        file_path: "src/main.rs".into(),
        modified: true,
        line: 42,
        column: 10,
        ..Default::default()
    };
    let out = render_tokens(&tokens, &vars);
    assert!(out.contains("src/main.rs"));
    assert!(out.contains("[+]"));
    assert!(out.contains("42:10"));
}

#[test]
fn render_readonly_hidden_when_false() {
    let tokens = parse_format("%r");
    let vars = DslVars::default();
    let out = render_tokens(&tokens, &vars);
    assert!(out.is_empty());
}

#[test]
fn render_percent_value() {
    let tokens = parse_format("%p%%");
    let vars = DslVars {
        percent: 75,
        ..Default::default()
    };
    let out = render_tokens(&tokens, &vars);
    assert_eq!(out, "75%");
}

#[test]
fn all_variables() {
    let tokens = parse_format("%f %F %m %r %l %c %p %y");
    let vars = DslVars {
        file_path: "a.rs".into(),
        file_path_abs: "/x/a.rs".into(),
        modified: true,
        readonly: true,
        line: 1,
        column: 2,
        percent: 50,
        file_type: "rust".into(),
    };
    let out = render_tokens(&tokens, &vars);
    assert!(out.contains("a.rs"));
    assert!(out.contains("/x/a.rs"));
    assert!(out.contains("[+]"));
    assert!(out.contains("[-]"));
    assert!(out.contains("rust"));
}
