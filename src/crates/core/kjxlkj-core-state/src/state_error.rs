use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("no such buffer")]
    NoSuchBuffer,
    #[error("no such window")]
    NoSuchWindow,
}

