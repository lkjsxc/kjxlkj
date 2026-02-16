/// Export and backup job domain types per /docs/spec/domain/export.md
///
/// Jobs track async export operations with lifecycle states:
/// queued → running → succeeded | failed
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Export job status per /docs/spec/domain/export.md
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportJobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

impl ExportJobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}

/// Export job kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportJobKind {
    /// Markdown export of current projections.
    Markdown,
    /// SQL backup dump.
    SqlBackup,
}

/// An export/backup job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJob {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub kind: ExportJobKind,
    pub status: ExportJobStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Artifact URL once succeeded.
    pub artifact_url: Option<String>,
    /// Error message if failed.
    pub error_message: Option<String>,
}

impl ExportJob {
    /// Create a new queued export job.
    pub fn new(workspace_id: Uuid, kind: ExportJobKind) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            workspace_id,
            kind,
            status: ExportJobStatus::Queued,
            created_at: now,
            updated_at: now,
            artifact_url: None,
            error_message: None,
        }
    }

    /// Transition to running.
    pub fn start(&mut self) {
        self.status = ExportJobStatus::Running;
        self.updated_at = Utc::now();
    }

    /// Transition to succeeded with artifact URL.
    pub fn succeed(&mut self, artifact_url: String) {
        self.status = ExportJobStatus::Succeeded;
        self.artifact_url = Some(artifact_url);
        self.updated_at = Utc::now();
    }

    /// Transition to failed with error message.
    pub fn fail(&mut self, error: String) {
        self.status = ExportJobStatus::Failed;
        self.error_message = Some(error);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_job_lifecycle() {
        let ws = Uuid::new_v4();
        let mut job = ExportJob::new(ws, ExportJobKind::Markdown);
        assert_eq!(job.status, ExportJobStatus::Queued);
        assert_eq!(job.workspace_id, ws);
        assert!(job.artifact_url.is_none());

        job.start();
        assert_eq!(job.status, ExportJobStatus::Running);

        job.succeed("/exports/test.zip".to_string());
        assert_eq!(job.status, ExportJobStatus::Succeeded);
        assert_eq!(job.artifact_url.as_deref(), Some("/exports/test.zip"));
    }

    #[test]
    fn test_export_job_failure() {
        let mut job = ExportJob::new(Uuid::new_v4(), ExportJobKind::SqlBackup);
        job.start();
        job.fail("disk full".to_string());
        assert_eq!(job.status, ExportJobStatus::Failed);
        assert_eq!(job.error_message.as_deref(), Some("disk full"));
    }

    #[test]
    fn test_export_status_as_str() {
        assert_eq!(ExportJobStatus::Queued.as_str(), "queued");
        assert_eq!(ExportJobStatus::Running.as_str(), "running");
        assert_eq!(ExportJobStatus::Succeeded.as_str(), "succeeded");
        assert_eq!(ExportJobStatus::Failed.as_str(), "failed");
    }

    #[test]
    fn test_export_job_serde_roundtrip() {
        let job = ExportJob::new(Uuid::new_v4(), ExportJobKind::Markdown);
        let json = serde_json::to_string(&job).unwrap();
        let back: ExportJob = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, job.id);
        assert_eq!(back.status, ExportJobStatus::Queued);
        assert_eq!(back.kind, ExportJobKind::Markdown);
    }
}
