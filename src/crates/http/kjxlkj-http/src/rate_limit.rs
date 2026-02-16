/// Rate limiting middleware per /docs/spec/security/auth.md
///
/// Per IMP-SEC-02: auth endpoint rate limiting for brute-force resistance.
/// Uses a per-IP sliding window counter stored in-memory.
///
/// Applied to: POST /api/setup/register, POST /api/auth/login
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests in the window
    pub max_requests: u32,
    /// Time window duration
    pub window: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 10,
            window: Duration::from_secs(60),
        }
    }
}

/// Per-key rate limit entry
#[derive(Debug)]
struct RateLimitEntry {
    timestamps: Vec<Instant>,
}

/// In-memory rate limiter using sliding window.
/// Thread-safe via interior Mutex.
#[derive(Debug)]
pub struct RateLimiter {
    config: RateLimitConfig,
    entries: Mutex<HashMap<String, RateLimitEntry>>,
}

impl RateLimiter {
    /// Create a new rate limiter with the given configuration.
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Check and record an attempt for the given key (e.g., IP address).
    /// Returns `Ok(remaining)` if allowed, `Err(retry_after_secs)` if rate limited.
    pub fn check(&self, key: &str) -> Result<u32, u64> {
        let now = Instant::now();
        let cutoff = now - self.config.window;
        let mut entries = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        let entry = entries.entry(key.to_string()).or_insert(RateLimitEntry {
            timestamps: Vec::new(),
        });
        // Prune expired timestamps
        entry.timestamps.retain(|t| *t > cutoff);
        if entry.timestamps.len() >= self.config.max_requests as usize {
            // Calculate retry-after from oldest entry in window
            let oldest = entry.timestamps.first().copied().unwrap_or(now);
            let retry_after = self
                .config
                .window
                .checked_sub(now.duration_since(oldest))
                .unwrap_or(Duration::from_secs(1));
            return Err(retry_after.as_secs().max(1));
        }
        entry.timestamps.push(now);
        let remaining = self.config.max_requests - entry.timestamps.len() as u32;
        Ok(remaining)
    }

    /// Evict stale entries (garbage collection).
    /// Call periodically to prevent unbounded memory growth.
    pub fn evict_stale(&self) {
        let cutoff = Instant::now() - self.config.window;
        let mut entries = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        entries.retain(|_k, v| {
            v.timestamps.retain(|t| *t > cutoff);
            !v.timestamps.is_empty()
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_within_limit() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 3,
            window: Duration::from_secs(60),
        });
        assert!(limiter.check("ip1").is_ok());
        assert!(limiter.check("ip1").is_ok());
        assert!(limiter.check("ip1").is_ok());
    }

    #[test]
    fn rejects_over_limit() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 2,
            window: Duration::from_secs(60),
        });
        assert!(limiter.check("ip1").is_ok());
        assert!(limiter.check("ip1").is_ok());
        assert!(limiter.check("ip1").is_err());
    }

    #[test]
    fn separate_keys_independent() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 1,
            window: Duration::from_secs(60),
        });
        assert!(limiter.check("ip1").is_ok());
        assert!(limiter.check("ip2").is_ok());
        assert!(limiter.check("ip1").is_err());
        assert!(limiter.check("ip2").is_err());
    }

    #[test]
    fn returns_remaining_count() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 5,
            window: Duration::from_secs(60),
        });
        assert_eq!(limiter.check("ip1").unwrap(), 4);
        assert_eq!(limiter.check("ip1").unwrap(), 3);
    }

    #[test]
    fn evict_stale_cleans_entries() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 10,
            window: Duration::from_millis(1),
        });
        limiter.check("ip1").unwrap();
        std::thread::sleep(Duration::from_millis(5));
        limiter.evict_stale();
        // After eviction, key should be gone, so limit resets
        assert!(limiter.check("ip1").is_ok());
    }

    #[test]
    fn retry_after_is_positive() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 1,
            window: Duration::from_secs(30),
        });
        limiter.check("ip1").unwrap();
        let err = limiter.check("ip1").unwrap_err();
        assert!(err >= 1);
    }
}
