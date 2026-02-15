//! kjxlkj-workspace: Workspace, project, view, and dashboard services.
//! Per /docs/spec/domain/workspaces.md, /docs/spec/domain/projects.md.
//! Thin service layer — most logic delegates to kjxlkj-db repositories.

pub use kjxlkj_db::repo::workspace::*;
