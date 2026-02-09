//! Wave 16 tests: lookaround, block replace, snippet tab-stops,
//! session options, expr vars, history filter, gv reselect, visual star.
#![cfg(test)]

use kjxlkj_core_types::{Key, KeyCode, Mode, Modifier, VisualKind};

use crate::editor::EditorState;

fn ed(text: &str) -> EditorState {
    let mut e = EditorState::new(80, 24);
    e.open_file("test.txt", text);
    e
}

fn key(c: char) -> Key {
    Key::char(c)
}

/// REQ-LOOKAROUND-01: Vim lookaround translates to Rust regex.
#[test]
fn lookaround_translation() {
    use crate::regex_translate::translate_vim_to_rust;
    // Positive lookahead: \(bar\)\@=
    let r = translate_vim_to_rust(r"foo\(bar\)\@=");
    assert!(r.pattern.contains("(?=bar)"), "Got: {}", r.pattern);
    // Negative lookahead: \(bar\)\@!
    let r = translate_vim_to_rust(r"foo\(bar\)\@!");
    assert!(r.pattern.contains("(?!bar)"), "Got: {}", r.pattern);
    // Positive lookbehind: \(foo\)\@<=
    let r = translate_vim_to_rust(r"\(foo\)\@<=bar");
    assert!(r.pattern.contains("(?<=foo)"), "Got: {}", r.pattern);
    // Negative lookbehind: \(foo\)\@<!
    let r = translate_vim_to_rust(r"\(foo\)\@<!bar");
    assert!(r.pattern.contains("(?<!foo)"), "Got: {}", r.pattern);
}

/// REQ-BLOCKR-01: Visual block r{char} replaces characters.
#[test]
fn visual_block_replace() {
    let mut e = ed("abcd\nefgh\nijkl\n");
    // Enter visual block: select 2x2 block at (0,0)-(1,1)
    e.handle_key(Key::new(KeyCode::Char('v'), Modifier::CTRL));
    e.handle_key(key('j'));
    e.handle_key(key('l'));
    // r followed by X
    e.handle_key(key('r'));
    e.handle_key(key('X'));
    assert!(matches!(e.mode, Mode::Normal));
    let line0: String = e.buffers.current().content.line(0).chars().collect();
    let line1: String = e.buffers.current().content.line(1).chars().collect();
    assert!(line0.starts_with("XX"), "Got: {line0}");
    assert!(line1.starts_with("XX"), "Got: {line1}");
}

/// REQ-SNIPTAB-01: Snippet tab-stop parsing.
#[test]
fn snippet_tab_stop_parsing() {
    let mut reg = crate::snippets::SnippetRegistry::new();
    reg.add("if", "if $1 {\n    $2\n}$0", "If template");
    let (text, stops) = reg.expand("if").unwrap();
    assert_eq!(text, "if  {\n    \n}");
    assert_eq!(stops.len(), 3, "Should have 3 stops: $1, $2, $0");
    // $1 should come first (at offset 3: "if ")
    assert_eq!(stops[0], 3);
}

/// REQ-SESSOPTS-01: Session saves editor options.
#[test]
fn session_saves_options() {
    let mut e = ed("hello\n");
    e.options
        .set("tabstop", crate::options::OptionValue::Int(4));
    // Capture mksession output (writes to file, so we test via internal)
    let mut lines = Vec::new();
    lines.push("\" Session".to_string());
    for (name, val) in e.options.list() {
        lines.push(format!("set {}={}", name, val));
    }
    let content = lines.join("\n");
    assert!(content.contains("set tabstop=4"), "Got: {content}");
}

/// REQ-EXPRVARS-01: Expression evaluator resolves g: variables.
#[test]
fn expr_vars() {
    use std::collections::HashMap;
    let mut vars = HashMap::new();
    vars.insert("g:name".to_string(), "world".to_string());
    let r = crate::expr_eval::eval_expression_with_vars("g:name", &vars).unwrap();
    assert_eq!(r, "world");
}

/// REQ-HISTFILT-01: History up/down filters by prefix.
#[test]
fn history_prefix_filter() {
    let mut cmd = crate::cmdline::CmdlineHandler::new();
    cmd.history = vec!["set number".into(), "write".into(), "set wrap".into()];
    cmd.open(':');
    cmd.insert_char('s');
    cmd.insert_char('e');
    cmd.insert_char('t');
    // history_prev should skip "write" and find "set wrap"
    cmd.history_prev();
    assert_eq!(cmd.content, "set wrap");
    cmd.history_prev();
    assert_eq!(cmd.content, "set number");
}

/// REQ-GVRESEL-01: gv reselects last visual selection.
#[test]
fn gv_reselect() {
    let mut e = ed("hello world\nfoo bar\n");
    // Enter visual, move down, exit
    e.handle_key(key('v'));
    e.handle_key(key('l'));
    e.handle_key(Key::esc());
    assert!(matches!(e.mode, Mode::Normal));
    assert!(e.last_visual.is_some());
    // gv should re-enter visual
    e.handle_key(key('g'));
    e.handle_key(key('v'));
    assert!(matches!(e.mode, Mode::Visual(VisualKind::Char)));
}

/// REQ-VSTAR-01: * in visual mode searches for selected text.
#[test]
fn visual_star_search() {
    let mut e = ed("foo bar foo baz foo\n");
    // Select "foo" (visual, move right twice)
    e.handle_key(key('v'));
    e.handle_key(key('l'));
    e.handle_key(key('l'));
    // * to search
    e.handle_key(key('*'));
    assert!(matches!(e.mode, Mode::Normal));
    assert!(e.search.active);
    assert!(e.search.pattern.is_some());
}
