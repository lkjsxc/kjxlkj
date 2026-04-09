use crate::web::db::ResourceKind;

pub fn live_label(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Note => "Live note",
        ResourceKind::Media => "Live media",
    }
}

pub fn open_live_label(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Note => "Open live note",
        ResourceKind::Media => "Open live media",
    }
}

pub fn delete_label(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Note => "Delete note",
        ResourceKind::Media => "Delete media",
    }
}

pub fn history_summary(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Note => "Browse the live note and saved snapshots.",
        ResourceKind::Media => "Browse the live media and saved snapshots.",
    }
}
