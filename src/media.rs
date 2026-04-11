//! Media derivative metadata and generation helpers

use image::{imageops::FilterType, DynamicImage};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use tokio::fs;
use tokio::process::Command;

const WEBP_CONTENT_TYPE: &str = "image/webp";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaVariants {
    pub card: Option<MediaVariant>,
    pub display: Option<MediaVariant>,
    pub poster: Option<MediaVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaVariant {
    pub key: String,
    pub content_type: String,
    pub byte_size: i64,
    pub width: i32,
    pub height: i32,
}

pub struct GeneratedVariant {
    pub name: &'static str,
    pub variant: MediaVariant,
    pub bytes: Vec<u8>,
}

impl MediaVariants {
    pub fn get(&self, name: &str) -> Option<&MediaVariant> {
        match name {
            "card" => self.card.as_ref(),
            "display" => self.display.as_ref(),
            "poster" => self.poster.as_ref(),
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.card.is_none() && self.display.is_none() && self.poster.is_none()
    }
}

pub fn media_variants_from_json(value: Option<Value>) -> Option<MediaVariants> {
    value
        .and_then(|value| serde_json::from_value(value).ok())
        .filter(|variants: &MediaVariants| !variants.is_empty())
}

pub fn media_variants_to_json(variants: &Option<MediaVariants>) -> Option<Value> {
    variants
        .as_ref()
        .filter(|variants| !variants.is_empty())
        .and_then(|variants| serde_json::to_value(variants).ok())
}

pub fn image_variants(id: &str, bytes: &[u8], quality: i64) -> Vec<GeneratedVariant> {
    let Ok(image) = image::load_from_memory(bytes) else {
        return Vec::new();
    };
    encode_image_variants(id, &image, quality)
}

pub async fn image_variants_from_path(
    id: &str,
    path: &Path,
    quality: i64,
) -> Vec<GeneratedVariant> {
    let Some(image) = decode_image_from_path(path).await else {
        return Vec::new();
    };
    encode_image_variants(id, &image, quality)
}

pub async fn video_stills_from_path(id: &str, path: &Path, quality: i64) -> Vec<GeneratedVariant> {
    let Some(image) = decode_frame_from_path(path).await else {
        return Vec::new();
    };
    [
        encode_resized(id, "card", &image, 640, quality),
        encode_webp(id, "poster", image, quality),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn encode_image_variants(id: &str, image: &DynamicImage, quality: i64) -> Vec<GeneratedVariant> {
    [("card", 640u32), ("display", 1400u32)]
        .into_iter()
        .filter_map(|(name, max_edge)| encode_resized(id, name, image, max_edge, quality))
        .collect()
}

async fn decode_image_from_path(path: &Path) -> Option<DynamicImage> {
    if let Ok(bytes) = fs::read(path).await {
        if let Ok(image) = image::load_from_memory(&bytes) {
            return Some(image);
        }
        if let Some(image) = crate::media_svg::decode_svg(&bytes, path) {
            return Some(image);
        }
    }
    decode_frame_from_path(path).await
}

async fn decode_frame_from_path(path: &Path) -> Option<DynamicImage> {
    let output = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-i",
            path.to_str()?,
            "-frames:v",
            "1",
            "-f",
            "image2pipe",
            "-vcodec",
            "png",
            "pipe:1",
        ])
        .output()
        .await
        .ok()?;
    if !output.status.success() || output.stdout.is_empty() {
        return None;
    }
    image::load_from_memory(&output.stdout).ok()
}

fn encode_resized(
    id: &str,
    name: &'static str,
    image: &DynamicImage,
    max_edge: u32,
    quality: i64,
) -> Option<GeneratedVariant> {
    let resized = image.resize(max_edge, max_edge, FilterType::Triangle);
    encode_webp(id, name, resized, quality)
}

fn encode_webp(
    id: &str,
    name: &'static str,
    image: DynamicImage,
    quality: i64,
) -> Option<GeneratedVariant> {
    let rgba = image.to_rgba8();
    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), rgba.width(), rgba.height());
    let bytes = encoder.encode(quality.clamp(1, 100) as f32).to_vec();
    Some(GeneratedVariant {
        name,
        variant: MediaVariant {
            key: format!("media/{id}/variants/{name}.webp"),
            content_type: WEBP_CONTENT_TYPE.to_string(),
            byte_size: bytes.len() as i64,
            width: rgba.width() as i32,
            height: rgba.height() as i32,
        },
        bytes,
    })
}
