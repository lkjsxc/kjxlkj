use crate::media::{image_variants, video_poster, GeneratedVariant, MediaVariants};
use crate::storage::Storage;
use crate::web::db::MediaFamily;
use tracing::warn;

pub fn build_variants(
    id: &str,
    family: MediaFamily,
    title: &str,
    bytes: &[u8],
    quality: i64,
) -> Vec<GeneratedVariant> {
    match family {
        MediaFamily::Image => image_variants(id, bytes, quality),
        MediaFamily::Video => video_poster(id, title, quality).into_iter().collect(),
    }
}

pub async fn store_variants(
    storage: &Storage,
    generated: &[GeneratedVariant],
) -> (Option<MediaVariants>, Vec<String>) {
    let mut variants = MediaVariants::default();
    let mut keys = Vec::new();
    for item in generated {
        let key = item.variant.key.clone();
        let result = storage
            .put_object(&key, item.bytes.clone(), &item.variant.content_type)
            .await;
        if let Err(error) = result {
            warn!(variant = item.name, key = %key, error = %error, "media derivative upload failed");
            continue;
        }
        assign_variant(&mut variants, item);
        keys.push(key);
    }
    ((!variants.is_empty()).then_some(variants), keys)
}

fn assign_variant(variants: &mut MediaVariants, item: &GeneratedVariant) {
    match item.name {
        "card" => variants.card = Some(item.variant.clone()),
        "display" => variants.display = Some(item.variant.clone()),
        "poster" => variants.poster = Some(item.variant.clone()),
        _ => {}
    }
}
