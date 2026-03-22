use std::path::PathBuf;
use std::process::Command;

use chrono::{DateTime, Utc};
use tokio::fs;

use crate::core::content::path_for_slug;
use crate::error::AppError;
use crate::web::state::{ArticleHistory, ArticleHistoryEntry};
use crate::web::stores::content::RuntimeContentStore;
use crate::web::stores::content_index::index_saved_article;

impl RuntimeContentStore {
    pub async fn article_history_impl(&self, slug: &str) -> Result<ArticleHistory, AppError> {
        let relative = self.relative_article_path(slug)?;
        let output = Command::new("git")
            .arg("--no-pager")
            .arg("log")
            .arg("--follow")
            .arg("--format=%H\t%cI\t%s")
            .arg("--")
            .arg(&relative)
            .current_dir(self.repo_root()?)
            .output()
            .map_err(|source| AppError::content_io(relative.clone(), source))?;
        if !output.status.success() {
            return Ok(ArticleHistory {
                slug: slug.to_owned(),
                entries: Vec::new(),
            });
        }
        Ok(ArticleHistory {
            slug: slug.to_owned(),
            entries: parse_history_entries(&output.stdout),
        })
    }

    pub async fn restore_article_version_impl(
        &self,
        slug: &str,
        commit_id: &str,
    ) -> Result<(), AppError> {
        let path = path_for_slug(self.app_state.filesystem.root(), slug)?;
        let relative = self.relative_article_path(slug)?;
        let spec = format!("{commit_id}:{relative}");
        let output = Command::new("git")
            .arg("show")
            .arg(spec)
            .current_dir(self.repo_root()?)
            .output()
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        if !output.status.success() {
            return Err(AppError::content_io(
                path.display().to_string(),
                std::io::Error::new(std::io::ErrorKind::NotFound, "history revision not found"),
            ));
        }
        let markdown = String::from_utf8_lossy(&output.stdout).to_string();
        fs::write(&path, &markdown)
            .await
            .map_err(|source| AppError::content_io(path.display().to_string(), source))?;
        index_saved_article(&self.app_state, slug, &markdown).await?;
        let updated_at = Utc::now();
        let created_at = self.read_or_assign_created_at(slug, updated_at).await?;
        self.sync_article_metadata(slug, created_at, updated_at)
            .await?;
        self.maybe_commit_history(slug, "restore")?;
        Ok(())
    }

    pub fn maybe_commit_history(&self, slug: &str, action: &str) -> Result<(), AppError> {
        let repo_root = self.repo_root()?;
        if !repo_root.join(".git").exists() {
            return Ok(());
        }
        let relative = self.relative_article_path(slug)?;
        if self.is_recent_commit(&repo_root, &relative)? {
            return Ok(());
        }
        if !self.stage_article(&repo_root, &relative)? {
            return Ok(());
        }
        self.commit_article_change(&repo_root, &relative, slug, action)
    }

    fn repo_root(&self) -> Result<PathBuf, AppError> {
        std::env::current_dir().map_err(|source| {
            AppError::content_io(
                self.app_state.filesystem.root().display().to_string(),
                source,
            )
        })
    }

    fn relative_article_path(&self, slug: &str) -> Result<String, AppError> {
        let repo_root = self.repo_root()?;
        let content_root = self.app_state.filesystem.root();
        let prefix = content_root.strip_prefix(&repo_root).map_err(|_| {
            AppError::content_io(
                content_root.display().to_string(),
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "content root outside repo",
                ),
            )
        })?;
        Ok(format!("{}/{}.md", prefix.display(), slug).replace('\\', "/"))
    }

    fn is_recent_commit(&self, repo_root: &PathBuf, relative: &str) -> Result<bool, AppError> {
        let now = Utc::now();
        let recent = Command::new("git")
            .arg("--no-pager")
            .arg("log")
            .arg("-1")
            .arg("--format=%ct")
            .arg("--")
            .arg(relative)
            .current_dir(repo_root)
            .output()
            .map_err(|source| AppError::content_io(relative.to_owned(), source))?;
        if !recent.status.success() {
            return Ok(false);
        }
        let last_ts = String::from_utf8_lossy(&recent.stdout).trim().to_owned();
        Ok(last_ts
            .parse::<i64>()
            .map(|epoch| now.timestamp().saturating_sub(epoch) < 60)
            .unwrap_or(false))
    }

    fn stage_article(&self, repo_root: &PathBuf, relative: &str) -> Result<bool, AppError> {
        let add = Command::new("git")
            .arg("add")
            .arg(relative)
            .current_dir(repo_root)
            .output()
            .map_err(|source| AppError::content_io(relative.to_owned(), source))?;
        Ok(add.status.success())
    }

    fn commit_article_change(
        &self,
        repo_root: &PathBuf,
        relative: &str,
        slug: &str,
        action: &str,
    ) -> Result<(), AppError> {
        let message = format!("article({slug}): {action}");
        let commit = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(&message)
            .arg("-m")
            .arg("Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>")
            .current_dir(repo_root)
            .output()
            .map_err(|source| AppError::content_io(relative.to_owned(), source))?;
        if !commit.status.success() {
            return Ok(());
        }
        Ok(())
    }
}

fn parse_history_entries(stdout: &[u8]) -> Vec<ArticleHistoryEntry> {
    let text = String::from_utf8_lossy(stdout);
    let mut entries = Vec::new();
    for line in text.lines() {
        let mut parts = line.splitn(3, '\t');
        let Some(commit_id) = parts.next() else {
            continue;
        };
        let Some(committed_at_raw) = parts.next() else {
            continue;
        };
        let Some(message) = parts.next() else {
            continue;
        };
        let Ok(committed_at) = DateTime::parse_from_rfc3339(committed_at_raw) else {
            continue;
        };
        entries.push(ArticleHistoryEntry {
            commit_id: commit_id.to_owned(),
            committed_at: committed_at.with_timezone(&Utc),
            message: message.to_owned(),
        });
    }
    entries
}
