use std::collections::HashMap;
use std::sync::Mutex;
use time::OffsetDateTime;

#[derive(Debug)]
pub struct FixedWindowRateLimiter {
    max_hits: u32,
    window_seconds: i64,
    buckets: Mutex<HashMap<String, Bucket>>,
}

#[derive(Debug, Clone, Copy)]
struct Bucket {
    window_start: OffsetDateTime,
    hits: u32,
}

impl FixedWindowRateLimiter {
    pub fn new(max_hits: u32, window_seconds: i64) -> Self {
        Self {
            max_hits,
            window_seconds,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    pub fn check(&self, key: &str) -> bool {
        let now = OffsetDateTime::now_utc();
        let mut buckets = self.buckets.lock().expect("rate limiter mutex poisoned");
        let bucket = buckets.entry(key.to_owned()).or_insert(Bucket {
            window_start: now,
            hits: 0,
        });

        if (now - bucket.window_start).whole_seconds() >= self.window_seconds {
            bucket.window_start = now;
            bucket.hits = 0;
        }

        if bucket.hits >= self.max_hits {
            return false;
        }

        bucket.hits += 1;
        true
    }
}
