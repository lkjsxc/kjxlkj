use crate::web::db::RecordKind;

pub fn live_label(kind: RecordKind) -> &'static str {
    match kind {
        RecordKind::Note => "Live note",
        RecordKind::Media => "Live media",
    }
}

pub fn open_live_label(kind: RecordKind) -> &'static str {
    match kind {
        RecordKind::Note => "Open live note",
        RecordKind::Media => "Open live media",
    }
}

pub fn delete_label(kind: RecordKind) -> &'static str {
    match kind {
        RecordKind::Note => "Delete note",
        RecordKind::Media => "Delete media",
    }
}

pub fn history_summary(kind: RecordKind) -> &'static str {
    match kind {
        RecordKind::Note => "Browse the live note and saved snapshots.",
        RecordKind::Media => "Browse the live media and saved snapshots.",
    }
}
