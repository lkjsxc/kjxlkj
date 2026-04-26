use super::password;
use super::DbPool;
use crate::error::AppError;
use uuid::Uuid;

pub async fn issue_password_reset_token(pool: &DbPool) -> Result<Option<String>, AppError> {
    let client = client(pool).await?;
    let Some(row) = client
        .query_opt("SELECT id FROM admin_user ORDER BY created_at LIMIT 1", &[])
        .await
        .map_err(db_error)?
    else {
        return Ok(None);
    };
    let token = new_token();
    client
        .execute(
            "INSERT INTO password_reset_tokens (user_id, token_hash, expires_at) \
             VALUES ($1, $2, NOW() + INTERVAL '15 minutes')",
            &[&row.get::<_, Uuid>("id"), &password::hash_secret(&token)?],
        )
        .await
        .map_err(db_error)?;
    Ok(Some(token))
}

pub async fn reset_admin_password(
    pool: &DbPool,
    token: &str,
    password: &str,
) -> Result<bool, AppError> {
    let mut db = client(pool).await?;
    let tx = db.transaction().await.map_err(db_error)?;
    let rows = tx
        .query(
            "SELECT id, user_id, token_hash FROM password_reset_tokens \
             WHERE used_at IS NULL AND expires_at > NOW() \
             ORDER BY created_at DESC FOR UPDATE",
            &[],
        )
        .await
        .map_err(db_error)?;
    let Some(row) = rows
        .into_iter()
        .find(|row| password::verify_secret(token.trim(), &row.get::<_, String>("token_hash")))
    else {
        return Ok(false);
    };
    let user_id: Uuid = row.get("user_id");
    update_password_in_tx(&tx, user_id, password).await?;
    tx.execute(
        "UPDATE password_reset_tokens SET used_at = NOW() WHERE id = $1",
        &[&row.get::<_, Uuid>("id")],
    )
    .await
    .map_err(db_error)?;
    tx.commit().await.map_err(db_error)?;
    Ok(true)
}

pub async fn update_admin_password(
    pool: &DbPool,
    user_id: Uuid,
    password: &str,
) -> Result<(), AppError> {
    let mut db = client(pool).await?;
    let tx = db.transaction().await.map_err(db_error)?;
    update_password_in_tx(&tx, user_id, password).await?;
    tx.commit().await.map_err(db_error)
}

pub async fn verify_admin_password(
    pool: &DbPool,
    user_id: Uuid,
    password: &str,
) -> Result<bool, AppError> {
    let Some(row) = client(pool)
        .await?
        .query_opt(
            "SELECT password_hash FROM admin_user WHERE id = $1",
            &[&user_id],
        )
        .await
        .map_err(db_error)?
    else {
        return Ok(false);
    };
    let hash: String = row.get("password_hash");
    Ok(password::verify_secret(password, &hash))
}

async fn update_password_in_tx(
    tx: &tokio_postgres::Transaction<'_>,
    user_id: Uuid,
    password: &str,
) -> Result<(), AppError> {
    let password_hash = password::hash_secret(password)?;
    tx.execute(
        "UPDATE admin_user SET password_hash = $2 WHERE id = $1",
        &[&user_id, &password_hash],
    )
    .await
    .map_err(db_error)?;
    tx.execute("DELETE FROM sessions WHERE user_id = $1", &[&user_id])
        .await
        .map(|_| ())
        .map_err(db_error)
}

fn new_token() -> String {
    format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple())
}

async fn client(pool: &DbPool) -> Result<deadpool_postgres::Object, AppError> {
    pool.get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

fn db_error(error: tokio_postgres::Error) -> AppError {
    AppError::DatabaseError(error.to_string())
}
