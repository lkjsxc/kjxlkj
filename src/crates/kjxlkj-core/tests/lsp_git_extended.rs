//! Tests for LSP extended protocol types and git conflict detection.

// --- LSP protocol_ext ---

#[test]
fn hover_with_markup_content() {
    use kjxlkj_service_lsp::protocol_ext::{Hover, HoverContents, MarkupContent, MarkupKind};
    let h = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown, value: "# fn main()".into(),
        }), range: None,
    };
    let j = serde_json::to_string(&h).unwrap();
    assert!(j.contains("markdown"));
    let parsed: Hover = serde_json::from_str(&j).unwrap();
    assert!(parsed.range.is_none());
}

#[test]
fn hover_with_marked_string() {
    use kjxlkj_service_lsp::protocol_ext::{Hover, HoverContents, MarkedString};
    let h = Hover {
        contents: HoverContents::MarkedString(MarkedString::LanguageString {
            language: "rust".into(), value: "fn foo()".into(),
        }), range: None,
    };
    let j = serde_json::to_string(&h).unwrap();
    assert!(j.contains("rust"));
}

#[test]
fn signature_help_roundtrip() {
    use kjxlkj_service_lsp::protocol_ext::*;
    let sh = SignatureHelp {
        signatures: vec![SignatureInformation {
            label: "fn bar(x: i32, y: &str)".into(),
            documentation: Some(MarkupContent { kind: MarkupKind::PlainText, value: "docs".into() }),
            parameters: Some(vec![
                ParameterInformation { label: ParameterLabel::Simple("x: i32".into()), documentation: None },
                ParameterInformation { label: ParameterLabel::Offsets([7, 13]), documentation: None },
            ]),
        }],
        active_signature: Some(0), active_parameter: Some(1),
    };
    let j = serde_json::to_string(&sh).unwrap();
    let parsed: SignatureHelp = serde_json::from_str(&j).unwrap();
    assert_eq!(parsed.signatures.len(), 1);
    assert_eq!(parsed.active_parameter, Some(1));
}

#[test]
fn code_action_response_serde() {
    use kjxlkj_service_lsp::protocol_ext::*;
    let ca = CodeActionResponse {
        title: "Add import".into(),
        kind: Some(code_action_kind::QUICK_FIX.into()),
        edit: Some(WorkspaceEdit {
            changes: Some({
                let mut m = std::collections::HashMap::new();
                m.insert("file:///test.rs".into(), vec![TextEditJson {
                    range: LspRangeJson {
                        start: LspPosJson { line: 0, character: 0 },
                        end: LspPosJson { line: 0, character: 0 },
                    }, new_text: "use std;\n".into(),
                }]); m
            }),
        }),
        command: None,
    };
    let j = serde_json::to_string(&ca).unwrap();
    assert!(j.contains("quickfix"));
    assert!(j.contains("use std;"));
}

#[test]
fn location_link_serde() {
    use kjxlkj_service_lsp::protocol_ext::*;
    let ll = LocationLink {
        origin_selection_range: None,
        target_uri: "file:///src/lib.rs".into(),
        target_range: LspRangeJson { start: LspPosJson { line: 10, character: 0 },
            end: LspPosJson { line: 15, character: 0 } },
        target_selection_range: LspRangeJson { start: LspPosJson { line: 10, character: 4 },
            end: LspPosJson { line: 10, character: 12 } },
    };
    let j = serde_json::to_string(&ll).unwrap();
    assert!(j.contains("lib.rs"));
}

#[test]
fn formatting_options_serde() {
    use kjxlkj_service_lsp::protocol_ext::*;
    let fo = FormattingOptions {
        tab_size: 4, insert_spaces: true,
        trim_trailing_whitespace: Some(true),
    };
    let j = serde_json::to_string(&fo).unwrap();
    assert!(j.contains("tabSize"));
    assert!(j.contains("insertSpaces"));
}

#[test]
fn document_symbol_nested() {
    use kjxlkj_service_lsp::protocol_ext::*;
    let sym = DocumentSymbol {
        name: "MyStruct".into(), kind: SymbolKind::Struct,
        range: LspRangeJson { start: LspPosJson { line: 0, character: 0 },
            end: LspPosJson { line: 10, character: 1 } },
        selection_range: LspRangeJson { start: LspPosJson { line: 0, character: 7 },
            end: LspPosJson { line: 0, character: 15 } },
        children: Some(vec![DocumentSymbol {
            name: "field".into(), kind: SymbolKind::Field,
            range: LspRangeJson { start: LspPosJson { line: 1, character: 4 },
                end: LspPosJson { line: 1, character: 20 } },
            selection_range: LspRangeJson { start: LspPosJson { line: 1, character: 4 },
                end: LspPosJson { line: 1, character: 9 } },
            children: None,
        }]),
    };
    let j = serde_json::to_string(&sym).unwrap();
    assert!(j.contains("MyStruct")); assert!(j.contains("field"));
}

#[test]
fn code_lens_serde() {
    use kjxlkj_service_lsp::protocol_ext::*;
    let cl = CodeLens {
        range: LspRangeJson { start: LspPosJson { line: 5, character: 0 },
            end: LspPosJson { line: 5, character: 0 } },
        command: Some(LspCommand { title: "Run Test".into(), command: "test.run".into(), arguments: None }),
    };
    let j = serde_json::to_string(&cl).unwrap();
    assert!(j.contains("Run Test"));
}

#[test]
fn rename_params_serde() {
    use kjxlkj_service_lsp::protocol_ext::*;
    use kjxlkj_service_lsp::protocol::TextDocumentIdentifier;
    let rp = RenameParams {
        text_document: TextDocumentIdentifier { uri: "file:///test.rs".into() },
        position: LspPosJson { line: 10, character: 5 },
        new_name: "new_name".into(),
    };
    let j = serde_json::to_string(&rp).unwrap();
    assert!(j.contains("new_name"));
}

// --- Git conflict detection ---

#[test]
fn detect_single_conflict() {
    use kjxlkj_service_git::conflict::BufferConflicts;
    let lines = vec![
        "normal", "<<<<<<< HEAD", "ours", "=======", "theirs", ">>>>>>> feature", "after",
    ];
    let bc = BufferConflicts::detect(&lines);
    assert_eq!(bc.len(), 1);
    assert_eq!(bc.conflicts[0].ours_start, 1);
    assert_eq!(bc.conflicts[0].theirs_end, 5);
}

#[test]
fn detect_two_conflicts() {
    use kjxlkj_service_git::conflict::BufferConflicts;
    let lines = vec![
        "<<<<<<< HEAD", "a", "=======", "b", ">>>>>>> x",
        "middle",
        "<<<<<<< HEAD", "c", "=======", "d", ">>>>>>> y",
    ];
    let bc = BufferConflicts::detect(&lines);
    assert_eq!(bc.len(), 2);
}

#[test]
fn file_indicator_symbols() {
    use kjxlkj_service_git::conflict::FileIndicator;
    assert_eq!(FileIndicator::Modified.symbol(), 'M');
    assert_eq!(FileIndicator::Added.symbol(), 'A');
    assert_eq!(FileIndicator::Deleted.symbol(), 'D');
    assert_eq!(FileIndicator::Renamed.symbol(), 'R');
    assert_eq!(FileIndicator::Untracked.symbol(), '?');
    assert_eq!(FileIndicator::Conflicted.symbol(), 'C');
    assert_eq!(FileIndicator::Ignored.symbol(), '!');
}

#[test]
fn diff_view_statistics() {
    use kjxlkj_service_git::conflict::{DiffView, DiffLine};
    use std::path::PathBuf;
    let mut dv = DiffView::new(PathBuf::from("file.rs"));
    dv.lines = vec![
        DiffLine::Header("@@ -1,3 +1,4 @@".into()),
        DiffLine::Context("unchanged".into()),
        DiffLine::Removed("old line".into()),
        DiffLine::Added("new line".into()),
        DiffLine::Added("extra line".into()),
    ];
    assert_eq!(dv.count_added(), 2);
    assert_eq!(dv.count_removed(), 1);
}

#[test]
fn diff_algorithm_variants() {
    use kjxlkj_service_git::conflict::{DiffAlgorithm, DiffOptions, DiffLayout};
    let opts = DiffOptions::default();
    assert_eq!(opts.algorithm, DiffAlgorithm::Myers);
    assert_ne!(DiffLayout::Unified, DiffLayout::SideBySide);
    assert_ne!(DiffLayout::Inline, DiffLayout::Unified);
}
