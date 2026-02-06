use kjxlkj_core_types::LanguageId;

#[test]
fn rust_extension() { assert_eq!(LanguageId::from_extension("rs"), LanguageId::Rust); }

#[test]
fn c_header() { assert_eq!(LanguageId::from_extension("h"), LanguageId::C); }

#[test]
fn cpp_extensions() {
    assert_eq!(LanguageId::from_extension("cpp"), LanguageId::Cpp);
    assert_eq!(LanguageId::from_extension("hpp"), LanguageId::Cpp);
}

#[test]
fn unknown_fallback() { assert_eq!(LanguageId::from_extension("xyz"), LanguageId::Plain); }

#[test]
fn dockerfile() { assert_eq!(LanguageId::from_filename("Dockerfile"), LanguageId::Dockerfile); }

#[test]
fn makefile() { assert_eq!(LanguageId::from_filename("Makefile"), LanguageId::Makefile); }

#[test]
fn filename_with_ext() { assert_eq!(LanguageId::from_filename("main.py"), LanguageId::Python); }

#[test]
fn lsp_id() {
    assert_eq!(LanguageId::Rust.lsp_id(), "rust");
    assert_eq!(LanguageId::Plain.lsp_id(), "plaintext");
}

#[test]
fn display_trait() { assert_eq!(format!("{}", LanguageId::Go), "go"); }
