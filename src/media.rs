//! Media derivative metadata and generation helpers

use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    [("card", 640u32), ("display", 1400u32)]
        .into_iter()
        .filter_map(|(name, max_edge)| encode_resized(id, name, &image, max_edge, quality))
        .collect()
}

pub fn video_poster(id: &str, title: &str, quality: i64) -> Option<GeneratedVariant> {
    let width = 640u32;
    let height = 360u32;
    let seed = title.bytes().fold(0u8, |acc, byte| acc.wrapping_add(byte));
    let mut image = ImageBuffer::from_pixel(width, height, Rgba([18, 24, 31, 255]));
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let band = ((x / 80 + y / 60) as u8).wrapping_add(seed);
        let blue = 90u8.saturating_add(band % 70);
        *pixel = Rgba([24 + band % 24, 38 + band % 32, blue, 255]);
    }
    let dynamic = DynamicImage::ImageRgba8(image);
    encode_webp(id, "poster", dynamic, quality)
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
