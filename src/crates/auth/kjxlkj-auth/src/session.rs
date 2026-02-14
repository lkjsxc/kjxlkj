// Session management per /docs/spec/security/sessions.md
use uuid::Uuid;

/// Generate a secure session token.
pub fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    hex::encode(&bytes)
}

/// Generate a CSRF token bound to session.
pub fn generate_csrf_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    hex::encode(&bytes)
}

/// Create a new session ID.
pub fn new_session_id() -> Uuid {
    Uuid::now_v7()
}

// hex encoding helper
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
}
