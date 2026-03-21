use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use kjxlkj::core::auth::AdminUser;
use kjxlkj::error::AppError;
use kjxlkj::web::state::AdminStore;

#[derive(Clone, Default)]
pub struct MockAdminStore {
    inner: Arc<Mutex<Option<AdminUser>>>,
}

impl MockAdminStore {
    pub fn set_admin(&self, username: &str, password_hash: &str) {
        let mut admin = self.inner.lock().expect("admin lock poisoned");
        *admin = Some(AdminUser {
            id: 1,
            username: username.to_owned(),
            password_hash: password_hash.to_owned(),
        });
    }
}

#[async_trait]
impl AdminStore for MockAdminStore {
    async fn has_admin_user(&self) -> Result<bool, AppError> {
        Ok(self.inner.lock().expect("admin lock poisoned").is_some())
    }

    async fn find_admin_by_username(&self, username: &str) -> Result<Option<AdminUser>, AppError> {
        let admin = self.inner.lock().expect("admin lock poisoned");
        Ok(admin.clone().filter(|value| value.username == username))
    }

    async fn create_admin(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<AdminUser, AppError> {
        let mut admin = self.inner.lock().expect("admin lock poisoned");
        let next = AdminUser {
            id: 1,
            username: username.to_owned(),
            password_hash: password_hash.to_owned(),
        };
        *admin = Some(next.clone());
        Ok(next)
    }
}
