//! Validation logic for note ids, aliases, and derived fields

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use thiserror::Error;
use uuid::Uuid;

static ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z2-7]{26}$").unwrap());
static ALIAS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z0-9]+(?:[-._][a-z0-9]+)*$").unwrap());
static ALIAS_SPACES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
static SUMMARY_PREFIX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?:[-+*]\s+|>\s+|\d+\.\s+|`{3,}[\w-]*\s*)").unwrap());
static RESERVED_ALIASES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "admin",
        "assets",
        "healthz",
        "login",
        "logout",
        "records",
        "robots.txt",
        "search",
        "setup",
        "sitemap.xml",
    ]
    .into_iter()
    .collect()
});

const BASE32_ALPHABET: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";
const ID_LEN: usize = 26;
const MAX_ALIAS_LEN: usize = 64;
const SUMMARY_LIMIT: usize = 120;
const SUMMARY_SUFFIX: &str = "...";

#[derive(Debug, Error, PartialEq)]
pub enum IdError {
    #[error("id must be exactly {ID_LEN} characters")]
    InvalidLength,
    #[error("id must be lowercase Base32")]
    InvalidFormat,
}

#[derive(Debug, Error, PartialEq)]
pub enum AliasError {
    #[error("alias must be 1 to {MAX_ALIAS_LEN} characters")]
    InvalidLength,
    #[error("alias must use lowercase letters, digits, and single separators: -, _, .")]
    InvalidFormat,
    #[error("alias is reserved")]
    Reserved,
    #[error("alias may not match the id format")]
    ConflictsWithId,
}

pub fn validate_id(id: &str) -> Result<(), IdError> {
    if id.len() != ID_LEN {
        return Err(IdError::InvalidLength);
    }
    if !ID_REGEX.is_match(id) {
        return Err(IdError::InvalidFormat);
    }
    Ok(())
}

pub fn generate_id() -> String {
    encode_base32(Uuid::new_v4().into_bytes())
}

pub fn normalize_alias(alias: Option<&str>) -> Result<Option<String>, AliasError> {
    let Some(alias) = alias
        .map(normalize_alias_input)
        .filter(|alias| !alias.is_empty())
    else {
        return Ok(None);
    };
    if alias.len() > MAX_ALIAS_LEN {
        return Err(AliasError::InvalidLength);
    }
    if looks_like_id(&alias) {
        return Err(AliasError::ConflictsWithId);
    }
    if RESERVED_ALIASES.contains(alias.as_str()) {
        return Err(AliasError::Reserved);
    }
    if !ALIAS_REGEX.is_match(&alias) {
        return Err(AliasError::InvalidFormat);
    }
    Ok(Some(alias.to_string()))
}

fn normalize_alias_input(alias: &str) -> String {
    ALIAS_SPACES
        .replace_all(&alias.trim().to_ascii_lowercase(), "-")
        .into_owned()
}

pub fn looks_like_id(value: &str) -> bool {
    ID_REGEX.is_match(value)
}

pub fn extract_title(body: &str) -> Option<String> {
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("# ") {
            return Some(title.to_string());
        }
        if trimmed.starts_with('#') || !trimmed.is_empty() {
            break;
        }
    }
    None
}

pub fn derive_title(body: &str) -> String {
    extract_title(body).unwrap_or_else(|| "Untitled note".to_string())
}

pub fn derive_summary(body: &str) -> String {
    let mut lines = meaningful_lines(body);
    let Some(first_line) = lines.next() else {
        return "No summary yet.".to_string();
    };
    shorten(&first_line, lines.next().is_some())
}

fn strip_summary_markers(line: &str) -> String {
    SUMMARY_PREFIX_REGEX.replace(line, "").trim().to_string()
}

fn meaningful_lines<'a>(body: &'a str) -> impl Iterator<Item = String> + 'a {
    body.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(strip_summary_markers)
        .filter(|line| !line.is_empty())
}

fn shorten(line: &str, has_more_content: bool) -> String {
    let line_len = line.chars().count();
    if line_len <= SUMMARY_LIMIT && !has_more_content {
        return line.to_string();
    }

    let suffix = SUMMARY_SUFFIX;
    let max_len = SUMMARY_LIMIT.saturating_sub(suffix.len());
    let prefix: String = line.chars().take(max_len).collect();
    format!("{}{suffix}", prefix.trim_end())
}

fn encode_base32(bytes: [u8; 16]) -> String {
    let mut output = String::with_capacity(ID_LEN);
    let mut buffer = 0u32;
    let mut bits = 0usize;

    for byte in bytes {
        buffer = (buffer << 8) | byte as u32;
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            output.push(BASE32_ALPHABET[((buffer >> bits) & 0x1f) as usize] as char);
        }
    }

    if bits > 0 {
        output.push(BASE32_ALPHABET[((buffer << (5 - bits)) & 0x1f) as usize] as char);
    }

    output
}
