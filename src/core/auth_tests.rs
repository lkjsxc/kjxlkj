#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};
    use uuid::Uuid;

    use crate::core::auth::{SessionRecord, SESSION_TTL_HOURS};

    #[test]
    fn session_record_uses_fixed_24_hour_ttl() {
        let created_at = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();
        let session = SessionRecord::new(Uuid::nil(), 7, created_at);

        assert_eq!(SESSION_TTL_HOURS, 24);
        assert_eq!(session.expires_at, created_at + Duration::hours(24));
    }

    #[test]
    fn session_expiry_check_is_deterministic() {
        let created_at = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();
        let session = SessionRecord::new(Uuid::nil(), 7, created_at);

        assert!(!session.is_expired_at(created_at + Duration::hours(23)));
        assert!(session.is_expired_at(created_at + Duration::hours(24)));
    }
}
