use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;
use walkdir::WalkDir;

#[derive(Clone, Serialize, Deserialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub private: bool,
}

#[derive(Clone)]
pub struct FsContentStore {
    root: PathBuf,
}

impl FsContentStore {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub async fn list(&self, include_private: bool) -> anyhow::Result<Vec<Article>> {
        let mut out = Vec::new();
        for entry in WalkDir::new(&self.root).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() || entry.path().extension().is_none_or(|x| x != "md") {
                continue;
            }
            if entry.path().file_name().is_some_and(|x| x == "README.md") {
                continue;
            }
            let rel = entry.path().strip_prefix(&self.root)?.with_extension("");
            let slug = rel.to_string_lossy().replace('\\', "/");
            let raw = fs::read_to_string(entry.path()).await?;
            let article = parse_article(&slug, &raw);
            if include_private || !article.private {
                out.push(article);
            }
        }
        out.sort_by(|a, b| a.slug.cmp(&b.slug));
        Ok(out)
    }

    pub async fn get(&self, slug: &str) -> anyhow::Result<Option<Article>> {
        let path = self.slug_path(slug)?;
        if !path.exists() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&path).await?;
        Ok(Some(parse_article(slug, &raw)))
    }

    pub async fn save(
        &self,
        slug: &str,
        title: &str,
        body: &str,
        private: bool,
    ) -> anyhow::Result<()> {
        let path = self.slug_path(slug)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let safe_title = title.replace(['\n', '\r'], " ").replace('"', "\\\"");
        let payload = format!("---\ntitle: \"{safe_title}\"\nprivate: {private}\n---\n{body}\n");
        let temp = path.with_extension("md.tmp");
        fs::write(&temp, payload).await?;
        fs::rename(temp, path).await?;
        Ok(())
    }

    pub async fn rename(&self, from_slug: &str, to_slug: &str) -> anyhow::Result<()> {
        let from = self.slug_path(from_slug)?;
        let to = self.slug_path(to_slug)?;
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::rename(from, to).await?;
        Ok(())
    }

    pub async fn delete(&self, slug: &str) -> anyhow::Result<()> {
        let path = self.slug_path(slug)?;
        fs::remove_file(path).await?;
        Ok(())
    }

    fn slug_path(&self, slug: &str) -> anyhow::Result<PathBuf> {
        if slug.trim().is_empty() {
            anyhow::bail!("invalid_slug_empty");
        }
        for part in slug.split('/') {
            if part.is_empty() || part == "." || part == ".." {
                anyhow::bail!("invalid_slug_path");
            }
            if !part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                anyhow::bail!("invalid_slug_chars");
            }
        }
        Ok(self.root.join(format!("{slug}.md")))
    }
}

fn parse_article(slug: &str, raw: &str) -> Article {
    let (title, private, body) = if let Some(stripped) = raw.strip_prefix("---\n") {
        if let Some(idx) = stripped.find("\n---\n") {
            let fm = &stripped[..idx];
            let body = stripped[(idx + 5)..].trim().to_string();
            let mut title = slug.to_string();
            let mut private = false;
            for line in fm.lines() {
                if let Some((k, v)) = line.split_once(':') {
                    let key = k.trim();
                    let value = v.trim().trim_matches('"').trim_matches('\'');
                    if key == "title" {
                        title = value.to_string();
                    } else if key == "private" {
                        private = value.eq_ignore_ascii_case("true");
                    }
                }
            }
            (title, private, body)
        } else {
            (slug.to_string(), false, raw.trim().to_string())
        }
    } else {
        (slug.to_string(), false, raw.trim().to_string())
    };

    Article {
        slug: slug.to_string(),
        title,
        body,
        private,
    }
}
