/// kjxlkj-db: Persistence layer with trait interfaces and in-memory implementations.
///
/// Canonical spec: /docs/spec/technical/migrations.md
/// Repository traits define the persistence contract. In-memory impls
/// provide a fully functional store for development and testing.
/// PostgreSQL-backed implementations use the same trait interfaces.
pub mod repo;
pub mod user_repo;
pub mod mem_note_repo;
pub mod mem_user_repo;
pub mod mem_workspace_repo;
pub mod mem_automation_repo;
pub mod mem_search_repo;
