//! Live streaming settings helpers

pub const LIVE_HEIGHTS: &[i64] = &[360, 480, 720, 1080, 1440, 2160];
pub const LIVE_FPS_VALUES: &[i64] = &[15, 30, 45, 60, 120];
pub const LIVE_CAMERA_FACING: &[&str] = &["environment", "user"];

pub fn normalize_live_source(value: &str) -> Result<String, String> {
    let value = value.trim();
    if matches!(value, "screen" | "camera") {
        Ok(value.to_string())
    } else {
        Err("Live default source must be screen or camera".to_string())
    }
}

pub fn normalize_live_camera_facing(value: &str) -> Result<String, String> {
    let value = value.trim();
    if LIVE_CAMERA_FACING.contains(&value) {
        Ok(value.to_string())
    } else {
        Err("Live default camera facing must be environment or user".to_string())
    }
}

pub fn validate_live_height(value: i64) -> Result<i64, String> {
    if LIVE_HEIGHTS.contains(&value) {
        Ok(value)
    } else {
        Err("Live default quality must use an approved height".to_string())
    }
}

pub fn validate_live_fps(value: i64) -> Result<i64, String> {
    if LIVE_FPS_VALUES.contains(&value) {
        Ok(value)
    } else {
        Err("Live default frame rate must use an approved fps".to_string())
    }
}
