use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct SetupRow {
    pub locked: bool,
}
