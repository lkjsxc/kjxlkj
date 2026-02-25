//! Agent run management

use uuid::Uuid;

use kjxlkj_domain::{AgentRun, RunStatus};
use kjxlkj_db::Result;

/// Run repository trait
pub trait RunRepo: Send + Sync {
    fn create_run(&self, run: AgentRun) -> impl Future<Output = Result<AgentRun>>;
    fn get_run(&self, run_id: Uuid) -> impl Future<Output = Result<Option<AgentRun>>>;
    fn list_runs(&self, workspace_id: Uuid) -> impl Future<Output = Result<Vec<AgentRun>>>;
    fn update_run(&self, run: AgentRun) -> impl Future<Output = Result<AgentRun>>;
}

use std::future::Future;
