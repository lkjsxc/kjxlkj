use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("db error")]
    Db,
}
