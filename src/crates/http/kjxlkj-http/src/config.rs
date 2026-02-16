use sqlx::PgPool;

/// Shared application configuration available to handlers.
#[derive(Clone)]
pub struct AppConfig {
    pub search_embedding_base_url: String,
    pub search_embedding_model: String,
    pub search_semantic_enabled: bool,
    pub agent_prompt_hash: String,
}

/// Shared application state.
pub struct AppState {
    pub pool: PgPool,
    pub config: AppConfig,
}
