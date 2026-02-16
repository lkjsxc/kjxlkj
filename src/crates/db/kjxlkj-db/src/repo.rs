/// Repository trait definitions for domain persistence.
///
/// Spec: /docs/spec/domain/events.md (transaction rule)
/// Spec: /docs/spec/domain/notes.md (write rules)
use kjxlkj_domain::note::*;
use kjxlkj_domain::workspace::*;
use kjxlkj_domain::event::*;
use kjxlkj_domain::search::*;
use kjxlkj_domain::automation::*;
use kjxlkj_domain::DomainError;
use uuid::Uuid;

/// Note repository operations  
pub trait NoteRepo: Send + Sync {
    fn create_note(
        &self,
        stream: &NoteStream,
        projection: &NoteProjection,
        event: &NoteEvent,
    ) -> Result<(), DomainError>;

    fn get_note_stream(&self, id: Uuid) -> Result<Option<NoteStream>, DomainError>;

    fn get_note_projection(&self, id: Uuid) -> Result<Option<NoteProjection>, DomainError>;

    fn list_notes(
        &self,
        workspace_id: Uuid,
        include_deleted: bool,
    ) -> Result<Vec<NoteStream>, DomainError>;

    fn update_note(
        &self,
        id: Uuid,
        base_version: i64,
        markdown: Option<&str>,
        title: Option<&str>,
        event: &NoteEvent,
    ) -> Result<NoteProjection, DomainError>;

    fn soft_delete_note(&self, id: Uuid, event: &NoteEvent) -> Result<(), DomainError>;

    fn get_note_history(&self, id: Uuid) -> Result<Vec<NoteEvent>, DomainError>;
}

/// Workspace repository operations
pub trait WorkspaceRepo: Send + Sync {
    fn create_workspace(&self, ws: &Workspace) -> Result<(), DomainError>;
    fn list_workspaces(&self, user_id: Uuid) -> Result<Vec<Workspace>, DomainError>;
    fn get_workspace(&self, id: Uuid) -> Result<Option<Workspace>, DomainError>;
}

/// Search repository operations
pub trait SearchRepo: Send + Sync {
    fn search_notes(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, DomainError>;
    fn get_backlinks(&self, note_id: Uuid) -> Result<Vec<Backlink>, DomainError>;
}

/// Automation repository operations
pub trait AutomationRepo: Send + Sync {
    fn create_rule(&self, rule: &AutomationRule) -> Result<(), DomainError>;
    fn list_rules(&self, workspace_id: Uuid) -> Result<Vec<AutomationRule>, DomainError>;
    fn update_rule(&self, rule: &AutomationRule) -> Result<(), DomainError>;
    fn create_run(&self, run: &AutomationRun) -> Result<(), DomainError>;
    fn list_runs(&self, workspace_id: Uuid) -> Result<Vec<AutomationRun>, DomainError>;
    fn get_run(&self, id: Uuid) -> Result<Option<AutomationRun>, DomainError>;
    fn update_run(&self, run: &AutomationRun) -> Result<(), DomainError>;
}
