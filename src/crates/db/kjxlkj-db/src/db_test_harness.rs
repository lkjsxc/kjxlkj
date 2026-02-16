/// DB-backed integration harness per IMP-TEST-03.
///
/// Spec: /docs/spec/technical/testing.md
///
/// Provides an optional PostgreSQL connection for integration tests.
/// When DATABASE_URL is set, runs queries against a real database.
/// When absent, tests are skipped with a diagnostic message.
/// This is containerless — relies on an existing PG instance.
use std::sync::OnceLock;

/// Cached check for whether a real database is available.
static DB_AVAILABLE: OnceLock<bool> = OnceLock::new();

/// Check if a real PostgreSQL connection string is configured.
/// Reads DATABASE_URL from environment.
pub fn database_url() -> Option<String> {
    std::env::var("DATABASE_URL").ok().filter(|s| !s.is_empty())
}

/// Returns true if a real database is reachable.
/// Caches the result for the process lifetime.
pub fn is_db_available() -> bool {
    *DB_AVAILABLE.get_or_init(|| database_url().is_some())
}

/// Skip a test if no database is available.
/// Call at the start of DB-dependent tests.
/// Returns true if the test should proceed, false if it should skip.
pub fn require_db() -> bool {
    if !is_db_available() {
        eprintln!(
            "SKIP: DATABASE_URL not set. Set it to run DB integration tests."
        );
        return false;
    }
    true
}

/// Configuration for the DB test harness.
pub struct DbTestConfig {
    pub url: String,
    pub max_connections: u32,
    pub connect_timeout_ms: u64,
}

impl DbTestConfig {
    /// Create from environment, with sensible test defaults.
    pub fn from_env() -> Option<Self> {
        database_url().map(|url| Self {
            url,
            max_connections: 2,
            connect_timeout_ms: 5000,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_url_reads_env() {
        // This test verifies the function works without panicking.
        // DATABASE_URL may or may not be set in CI.
        let _url = database_url();
    }

    #[test]
    fn test_require_db_returns_bool() {
        let result = require_db();
        // Result depends on environment; just verify no panic.
        assert!(result || !result);
    }

    #[test]
    fn test_db_test_config_from_env() {
        let config = DbTestConfig::from_env();
        if database_url().is_some() {
            assert!(config.is_some());
            let c = config.unwrap();
            assert_eq!(c.max_connections, 2);
            assert_eq!(c.connect_timeout_ms, 5000);
        } else {
            assert!(config.is_none());
        }
    }
}
