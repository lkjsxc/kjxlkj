use super::{encoded, frame_card, segment, segments};
use crate::core::MarkdownOptions;
use url::Url;

pub(super) fn render(url: &Url, host: &str, options: MarkdownOptions<'_>) -> Option<String> {
    match host {
        "youtube.com" | "m.youtube.com" | "youtube-nocookie.com" | "youtu.be" => youtube(url),
        "vimeo.com" | "player.vimeo.com" => vimeo(url),
        "soundcloud.com" => Some(frame_card("SoundCloud", url.as_str(), &soundcloud(url))),
        "open.spotify.com" => spotify(url),
        "tiktok.com" | "vm.tiktok.com" => tiktok(url),
        "dailymotion.com" | "dai.ly" => dailymotion(url),
        "twitch.tv" | "clips.twitch.tv" => twitch(url, options.public_base_url),
        "codepen.io" => codepen(url),
        "google.com" | "maps.google.com" => google_maps(url, options.google_maps_embed_api_key),
        _ => None,
    }
}

fn youtube(url: &Url) -> Option<String> {
    let host = url.host_str()?.trim_start_matches("www.");
    let video = if host == "youtu.be" {
        segment(url, 0).map(str::to_string)
    } else if url.path() == "/watch" {
        url.query_pairs()
            .find(|(key, _)| key == "v")
            .map(|(_, value)| value.to_string())
    } else if matches!(segment(url, 0), Some("shorts" | "embed")) {
        segment(url, 1).map(str::to_string)
    } else {
        None
    };
    if let Some(id) = video {
        return Some(frame_card(
            "YouTube",
            url.as_str(),
            &format!("https://www.youtube-nocookie.com/embed/{id}"),
        ));
    }
    url.query_pairs()
        .find(|(key, _)| key == "list")
        .map(|(_, list)| {
            frame_card(
                "YouTube",
                url.as_str(),
                &format!("https://www.youtube-nocookie.com/embed/videoseries?list={list}"),
            )
        })
}

fn vimeo(url: &Url) -> Option<String> {
    segment(url, 0)
        .filter(|value| value.chars().all(|ch| ch.is_ascii_digit()))
        .map(|id| {
            frame_card(
                "Vimeo",
                url.as_str(),
                &format!("https://player.vimeo.com/video/{id}"),
            )
        })
}

fn spotify(url: &Url) -> Option<String> {
    let parts = segments(url);
    matches!(
        parts.as_slice(),
        [kind @ ("track" | "album" | "playlist" | "episode" | "show" | "artist"), id, ..] if !id.is_empty()
    )
    .then(|| {
        frame_card(
            "Spotify",
            url.as_str(),
            &format!("https://open.spotify.com/embed/{}/{}", parts[0], parts[1]),
        )
    })
}

fn tiktok(url: &Url) -> Option<String> {
    let parts = segments(url);
    parts
        .windows(2)
        .find(|pair| pair[0] == "video" && pair[1].chars().all(|ch| ch.is_ascii_digit()))
        .map(|pair| {
            frame_card(
                "TikTok",
                url.as_str(),
                &format!("https://www.tiktok.com/player/v1/{}", pair[1]),
            )
        })
}

fn dailymotion(url: &Url) -> Option<String> {
    let id = if url.host_str()?.trim_start_matches("www.") == "dai.ly" {
        segment(url, 0)
    } else if segment(url, 0) == Some("video") {
        segment(url, 1)
    } else {
        None
    }?;
    Some(frame_card(
        "Dailymotion",
        url.as_str(),
        &format!("https://www.dailymotion.com/embed/video/{id}"),
    ))
}

fn twitch(url: &Url, public_base_url: Option<&str>) -> Option<String> {
    let parent = Url::parse(public_base_url?).ok()?.host_str()?.to_string();
    let clip = if url.host_str()?.trim_start_matches("www.") == "clips.twitch.tv" {
        segment(url, 0)
    } else if segment(url, 0) == Some("clip") {
        segment(url, 1)
    } else {
        None
    };
    if let Some(slug) = clip {
        return Some(frame_card(
            "Twitch",
            url.as_str(),
            &format!("https://clips.twitch.tv/embed?clip={slug}&parent={parent}"),
        ));
    }
    (segment(url, 0) == Some("videos")).then(|| {
        frame_card(
            "Twitch",
            url.as_str(),
            &format!(
                "https://player.twitch.tv/?video={}&parent={parent}",
                segment(url, 1).unwrap_or("")
            ),
        )
    })
}

fn codepen(url: &Url) -> Option<String> {
    let parts = segments(url);
    matches!(parts.as_slice(), [_, "pen", _, ..]).then(|| {
        frame_card(
            "CodePen",
            url.as_str(),
            &format!(
                "https://codepen.io/{}/embed/{}?default-tab=result",
                parts[0], parts[2]
            ),
        )
    })
}

fn google_maps(url: &Url, key: Option<&str>) -> Option<String> {
    let key = key?.trim();
    if key.is_empty() || !url.path().starts_with("/maps") {
        return None;
    }
    let query = url
        .query_pairs()
        .find(|(name, _)| matches!(name.as_ref(), "q" | "query"))
        .map(|(_, value)| value.to_string())
        .unwrap_or_else(|| url.path().trim_start_matches("/maps").replace('/', " "));
    Some(frame_card(
        "Google Maps",
        url.as_str(),
        &format!(
            "https://www.google.com/maps/embed/v1/search?key={}&q={}",
            encoded(key),
            encoded(query.trim())
        ),
    ))
}

fn soundcloud(url: &Url) -> String {
    format!(
        "https://w.soundcloud.com/player/?url={}",
        encoded(url.as_str())
    )
}
