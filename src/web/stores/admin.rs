use async_trait::async_trait;

use crate::app_state::AppState;
use crate::core::auth::AdminUser;
use crate::error::AppError;
use crate::web::state::AdminStore;

#[derive(Clone)]
pub struct RuntimeAdminStore {
    pub app_state: AppState,
}

#[async_trait]
impl AdminStore for RuntimeAdminStore {
    async fn has_admin_user(&self) -> Result<bool, AppError> {
        self.app_state.postgres.has_admin_user().await
    }

    async fn find_admin_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError> {
        self.app_state
            .postgres
            .admins()
            .find_by_username(username)
            .await
    }

    async fn create_admin(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<AdminUser, AppError> {
        self.app_state
            .postgres
            .admins()
            .create(username, password_hash)
            .await
    }
}
