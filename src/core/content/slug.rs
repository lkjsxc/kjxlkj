use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::errors::ContentValidationError;

pub fn validate_slug(slug: &str) -> Result<(), ContentValidationError> {
    if slug.is_empty() || slug.starts_with('-') || slug.ends_with('-') {
        return Err(ContentValidationError::InvalidSlug {
            value: slug.to_owned(),
        });
    }

    let mut previous_dash = false;
    for character in slug.chars() {
        let allowed =
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-';
        if !allowed {
            return Err(ContentValidationError::InvalidSlug {
                value: slug.to_owned(),
            });
        }

        if character == '-' {
            if previous_dash {
                return Err(ContentValidationError::InvalidSlug {
                    value: slug.to_owned(),
                });
            }
            previous_dash = true;
        } else {
            previous_dash = false;
        }
    }

    Ok(())
}

pub fn slug_from_stem(stem: &str) -> Result<String, ContentValidationError> {
    validate_slug(stem)?;
    Ok(stem.to_owned())
}

pub fn path_for_slug(root: &Path, slug: &str) -> Result<PathBuf, ContentValidationError> {
    validate_slug(slug)?;
    Ok(root.join(format!("{slug}.md")))
}

pub fn ensure_unique_slug(
    slug: &str,
    seen: &mut HashSet<String>,
) -> Result<(), ContentValidationError> {
    if seen.insert(slug.to_owned()) {
        Ok(())
    } else {
        Err(ContentValidationError::DuplicateSlug {
            slug: slug.to_owned(),
        })
    }
}

pub fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
}
