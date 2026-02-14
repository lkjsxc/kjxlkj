// User repository
use kjxlkj_domain::types::{Role, User, UserStatus};
use sqlx::PgPool;
use uuid::Uuid;

/// Check if any owner account exists (setup lock gate).
pub async fn owner_exists(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE role = 'owner'",
    )
    .fetch_one(pool)
    .await?;
    Ok(row.0 > 0)
}

/// Insert a new user.
pub async fn insert_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    let role_str = role_to_str(user.role);
    let status_str = status_to_str(user.status);
    sqlx::query(
        "INSERT INTO users (id, email, display_name, role, status, password_hash, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, now())",
    )
    .bind(user.id)
    .bind(&user.email)
    .bind(&user.display_name)
    .bind(role_str)
    .bind(status_str)
    .bind(&user.password_hash)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find user by email.
pub async fn find_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let row: Option<(Uuid, String, String, String, String, String)> = sqlx::query_as(
        "SELECT id, email, display_name, role, status, password_hash
         FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(to_user))
}

/// Find user by ID.
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let row: Option<(Uuid, String, String, String, String, String)> = sqlx::query_as(
        "SELECT id, email, display_name, role, status, password_hash
         FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(to_user))
}

/// List all users.
pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let rows: Vec<(Uuid, String, String, String, String, String)> = sqlx::query_as(
        "SELECT id, email, display_name, role, status, password_hash
         FROM users ORDER BY created_at",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(to_user).collect())
}

/// Update user role.
pub async fn update_role(pool: &PgPool, id: Uuid, role: Role) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(role_to_str(role))
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Disable user.
pub async fn disable_user(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE users SET status = 'disabled' WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

fn role_to_str(r: Role) -> &'static str {
    match r {
        Role::Owner => "owner",
        Role::Admin => "admin",
        Role::Editor => "editor",
        Role::Viewer => "viewer",
    }
}

fn status_to_str(s: UserStatus) -> &'static str {
    match s {
        UserStatus::Active => "active",
        UserStatus::Disabled => "disabled",
    }
}

fn to_user(row: (Uuid, String, String, String, String, String)) -> User {
    let role = match row.3.as_str() {
        "owner" => Role::Owner,
        "admin" => Role::Admin,
        "editor" => Role::Editor,
        _ => Role::Viewer,
    };
    let status = match row.4.as_str() {
        "disabled" => UserStatus::Disabled,
        _ => UserStatus::Active,
    };
    User {
        id: row.0,
        email: row.1,
        display_name: row.2,
        role,
        status,
        password_hash: row.5,
        created_at: String::new(),
    }
}
