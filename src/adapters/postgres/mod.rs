pub mod admin_repo;
pub mod session_repo;

use sqlx::postgres::PgPoolOptions;

use crate::error::AppError;

use self::admin_repo::AdminRepository;
use self::session_repo::SessionRepository;

#[derive(Debug, Clone)]
pub struct PostgresAdapter {
    admin_repo: AdminRepository,
    session_repo: SessionRepository,
}

impl PostgresAdapter {
    pub fn new(database_url: String) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(&database_url)
            .map_err(AppError::database_connect)?;

        Ok(Self {
            admin_repo: AdminRepository::new(pool.clone()),
            session_repo: SessionRepository::new(pool),
        })
    }

    pub fn admins(&self) -> &AdminRepository {
        &self.admin_repo
    }

    pub fn sessions(&self) -> &SessionRepository {
        &self.session_repo
    }

    pub async fn has_admin_user(&self) -> Result<bool, AppError> {
        self.admin_repo.has_any_admin().await
    }
}
