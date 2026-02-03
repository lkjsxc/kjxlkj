use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextError {
    #[error("index out of bounds")]
    IndexOutOfBounds,
}

