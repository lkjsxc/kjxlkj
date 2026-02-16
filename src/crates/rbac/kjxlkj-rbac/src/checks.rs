/// RBAC permission checks per /docs/spec/domain/permissions.md
use kjxlkj_domain::permission::Role;
use kjxlkj_domain::DomainError;

/// Require at least the given role.
pub fn require_role(actual: Role, minimum: Role) -> Result<(), DomainError> {
    if actual >= minimum {
        Ok(())
    } else {
        Err(DomainError::RoleForbidden)
    }
}

/// Check write permission (editor+)
pub fn require_write(role: Role) -> Result<(), DomainError> {
    if role.can_write() {
        Ok(())
    } else {
        Err(DomainError::RoleForbidden)
    }
}

/// Check management permission (admin+)
pub fn require_manage(role: Role) -> Result<(), DomainError> {
    if role.can_manage() {
        Ok(())
    } else {
        Err(DomainError::RoleForbidden)
    }
}

/// Check owner-only permission
pub fn require_owner(role: Role) -> Result<(), DomainError> {
    if role.is_owner() {
        Ok(())
    } else {
        Err(DomainError::RoleForbidden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_role() {
        assert!(require_role(Role::Owner, Role::Viewer).is_ok());
        assert!(require_role(Role::Viewer, Role::Owner).is_err());
        assert!(require_role(Role::Editor, Role::Editor).is_ok());
    }

    #[test]
    fn test_require_write() {
        assert!(require_write(Role::Editor).is_ok());
        assert!(require_write(Role::Viewer).is_err());
    }

    #[test]
    fn test_require_manage() {
        assert!(require_manage(Role::Admin).is_ok());
        assert!(require_manage(Role::Editor).is_err());
    }

    #[test]
    fn test_require_owner() {
        assert!(require_owner(Role::Owner).is_ok());
        assert!(require_owner(Role::Admin).is_err());
    }
}
