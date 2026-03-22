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

    async fn load_admin(&self) -> Result<Option<AdminUser>, AppError> {
        self.app_state.postgres.admins().load_fixed_admin().await
    }

    async fn create_admin(&self, password_hash: &str) -> Result<AdminUser, AppError> {
        self.app_state
            .postgres
            .admins()
            .create_fixed_admin(password_hash)
            .await
    }
}
