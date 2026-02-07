use kjxlkj_service_index::{
    fuzzy_match, normalize_score, rank_candidates, SymbolEntry, SymbolIndex, SymbolKind,
};

// --- Fuzzy matching ---

#[test]
fn fuzzy_exact_match() {
    let m = fuzzy_match("abc", "abc").unwrap();
    assert_eq!(m.indices, vec![0, 1, 2]);
    assert!(m.score > 0);
}

#[test]
fn fuzzy_no_match() {
    assert!(fuzzy_match("xyz", "abc").is_none());
}

#[test]
fn fuzzy_empty_pattern() {
    let m = fuzzy_match("", "anything").unwrap();
    assert_eq!(m.score, 0);
    assert!(m.indices.is_empty());
}

#[test]
fn fuzzy_case_insensitive() {
    let m = fuzzy_match("abc", "ABC").unwrap();
    assert_eq!(m.indices, vec![0, 1, 2]);
}

#[test]
fn fuzzy_subsequence() {
    let m = fuzzy_match("fb", "foo_bar").unwrap();
    assert!(m.indices.len() == 2);
}

#[test]
fn fuzzy_scoring_exact_case_bonus() {
    let lower = fuzzy_match("a", "a").unwrap();
    let upper = fuzzy_match("a", "A").unwrap();
    assert!(lower.score >= upper.score);
}

// --- Normalize score ---

#[test]
fn normalize_positive() {
    let n = normalize_score(5, 10);
    assert!(n > 0.0 && n <= 1.0);
}

#[test]
fn normalize_zero_len() {
    assert_eq!(normalize_score(5, 0), 0.0);
}

// --- Rank candidates ---

#[test]
fn rank_best_first() {
    let candidates = vec!["foo_bar", "foobar", "fb", "xfb"];
    let ranked = rank_candidates("fb", &candidates);
    assert!(!ranked.is_empty());
    assert_eq!(ranked[0].0, 2); // "fb" is best
}

#[test]
fn rank_no_matches() {
    let candidates = vec!["aaa", "bbb"];
    let ranked = rank_candidates("xyz", &candidates);
    assert!(ranked.is_empty());
}

// --- Symbol index ---

fn sample_index() -> SymbolIndex {
    let mut idx = SymbolIndex::new();
    idx.add(SymbolEntry {
        name: "foo_bar".into(),
        kind: SymbolKind::Function,
        file: "a.rs".into(),
        line: 10,
    });
    idx.add(SymbolEntry {
        name: "FooService".into(),
        kind: SymbolKind::Struct,
        file: "b.rs".into(),
        line: 1,
    });
    idx.add(SymbolEntry {
        name: "baz_qux".into(),
        kind: SymbolKind::Function,
        file: "c.rs".into(),
        line: 20,
    });
    idx
}

#[test]
fn symbol_index_add_query() {
    let idx = sample_index();
    assert_eq!(idx.entries.len(), 3);
}

#[test]
fn symbol_search_prefix() {
    let idx = sample_index();
    let results = idx.search("foo");
    assert_eq!(results.len(), 2);
}

#[test]
fn symbol_search_case_insensitive() {
    let idx = sample_index();
    let results = idx.search("FOO");
    assert_eq!(results.len(), 2);
}

#[test]
fn symbol_search_empty() {
    let idx = sample_index();
    let results = idx.search("");
    assert_eq!(results.len(), 3);
}

#[test]
fn symbol_search_no_match() {
    let idx = sample_index();
    let results = idx.search("zzz");
    assert!(results.is_empty());
}

#[test]
fn symbol_search_fuzzy() {
    let idx = sample_index();
    let results = idx.search_fuzzy("fb");
    assert!(!results.is_empty());
    assert_eq!(results[0].name, "foo_bar");
}

#[test]
fn symbol_kinds_distinct() {
    assert_ne!(SymbolKind::Function, SymbolKind::Struct);
    assert_ne!(SymbolKind::Class, SymbolKind::Enum);
}
