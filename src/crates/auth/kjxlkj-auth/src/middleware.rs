use uuid::Uuid;

/// Authenticated identity extracted from session.
#[derive(Debug, Clone)]
pub struct AuthIdentity {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub is_owner: bool,
    pub csrf_token: String,
}
