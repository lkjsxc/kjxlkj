/// In-memory export job repository per /docs/spec/domain/export.md
///
/// Stores export/backup jobs in a HashMap protected by RwLock.
/// Provides create, get, list, and update operations.
use kjxlkj_domain::export::ExportJob;
use kjxlkj_domain::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// Export job repository trait.
pub trait ExportRepo: Send + Sync {
    fn create_job(&self, job: &ExportJob) -> Result<(), DomainError>;
    fn get_job(&self, id: Uuid) -> Result<Option<ExportJob>, DomainError>;
    fn list_jobs(&self, workspace_id: Uuid) -> Result<Vec<ExportJob>, DomainError>;
    fn update_job(&self, job: &ExportJob) -> Result<(), DomainError>;
}

/// In-memory implementation of ExportRepo.
pub struct InMemoryExportRepo {
    jobs: RwLock<HashMap<Uuid, ExportJob>>,
}

impl InMemoryExportRepo {
    pub fn new() -> Self {
        Self {
            jobs: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryExportRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportRepo for InMemoryExportRepo {
    fn create_job(&self, job: &ExportJob) -> Result<(), DomainError> {
        let mut jobs = self
            .jobs
            .write()
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        jobs.insert(job.id, job.clone());
        Ok(())
    }

    fn get_job(&self, id: Uuid) -> Result<Option<ExportJob>, DomainError> {
        let jobs = self
            .jobs
            .read()
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(jobs.get(&id).cloned())
    }

    fn list_jobs(&self, workspace_id: Uuid) -> Result<Vec<ExportJob>, DomainError> {
        let jobs = self
            .jobs
            .read()
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let mut result: Vec<ExportJob> = jobs
            .values()
            .filter(|j| j.workspace_id == workspace_id)
            .cloned()
            .collect();
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(result)
    }

    fn update_job(&self, job: &ExportJob) -> Result<(), DomainError> {
        let mut jobs = self
            .jobs
            .write()
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        if !jobs.contains_key(&job.id) {
            return Err(DomainError::ExportNotFound);
        }
        jobs.insert(job.id, job.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_domain::export::{ExportJobKind, ExportJobStatus};

    #[test]
    fn test_create_and_get_job() {
        let repo = InMemoryExportRepo::new();
        let ws_id = Uuid::new_v4();
        let job = ExportJob::new(ws_id, ExportJobKind::Markdown);
        let job_id = job.id;
        repo.create_job(&job).unwrap();

        let found = repo.get_job(job_id).unwrap().unwrap();
        assert_eq!(found.id, job_id);
        assert_eq!(found.status, ExportJobStatus::Queued);
    }

    #[test]
    fn test_list_jobs_filtered_by_workspace() {
        let repo = InMemoryExportRepo::new();
        let ws1 = Uuid::new_v4();
        let ws2 = Uuid::new_v4();
        repo.create_job(&ExportJob::new(ws1, ExportJobKind::Markdown))
            .unwrap();
        repo.create_job(&ExportJob::new(ws1, ExportJobKind::SqlBackup))
            .unwrap();
        repo.create_job(&ExportJob::new(ws2, ExportJobKind::Markdown))
            .unwrap();

        let ws1_jobs = repo.list_jobs(ws1).unwrap();
        assert_eq!(ws1_jobs.len(), 2);
        let ws2_jobs = repo.list_jobs(ws2).unwrap();
        assert_eq!(ws2_jobs.len(), 1);
    }

    #[test]
    fn test_update_job() {
        let repo = InMemoryExportRepo::new();
        let mut job = ExportJob::new(Uuid::new_v4(), ExportJobKind::Markdown);
        repo.create_job(&job).unwrap();

        job.start();
        repo.update_job(&job).unwrap();
        let found = repo.get_job(job.id).unwrap().unwrap();
        assert_eq!(found.status, ExportJobStatus::Running);

        job.succeed("/out/export.zip".to_string());
        repo.update_job(&job).unwrap();
        let found = repo.get_job(job.id).unwrap().unwrap();
        assert_eq!(found.status, ExportJobStatus::Succeeded);
    }

    #[test]
    fn test_update_nonexistent_returns_not_found() {
        let repo = InMemoryExportRepo::new();
        let job = ExportJob::new(Uuid::new_v4(), ExportJobKind::Markdown);
        let result = repo.update_job(&job);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_nonexistent_returns_none() {
        let repo = InMemoryExportRepo::new();
        assert!(repo.get_job(Uuid::new_v4()).unwrap().is_none());
    }
}
