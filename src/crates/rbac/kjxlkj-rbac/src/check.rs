use kjxlkj_domain::permission::Role;
use kjxlkj_db::repo_workspace;
use sqlx::PgPool;
use uuid::Uuid;

/// Check user's role in a workspace. Returns None if not a member.
pub async fn get_role(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
) -> Result<Option<Role>, sqlx::Error> {
    let role_str = repo_workspace::get_member_role(pool, workspace_id, user_id).await?;
    Ok(role_str.and_then(|r| Role::from_str_checked(&r)))
}

/// Require at least `min_role` in workspace. Returns Err with
/// message if unauthorized.
pub async fn require_role(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
    min_role: Role,
) -> Result<Role, RbacError> {
    let role = get_role(pool, workspace_id, user_id)
        .await
        .map_err(RbacError::Db)?;

    match role {
        Some(r) if r >= min_role => Ok(r),
        Some(_) => Err(RbacError::Forbidden),
        None => Err(RbacError::NotMember),
    }
}

#[derive(Debug)]
pub enum RbacError {
    Forbidden,
    NotMember,
    Db(sqlx::Error),
}
