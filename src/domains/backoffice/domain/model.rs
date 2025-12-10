use super::super::role::model::Role;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_active: bool,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String, role: Role) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            is_active: true,
            role,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn has_role(&self, role_id: Uuid) -> bool {
        self.role.role_id == role_id
    }

    pub fn update_username(&mut self, username: String) {
        self.username = username;
        self.updated_at = Utc::now();
    }

    pub fn update_email(&mut self, email: String) {
        self.email = email;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::role::model::{admin_role_id, user_role_id};
    use super::*;

    fn create_test_user_role() -> Role {
        Role::new(
            user_role_id(),
            "User".to_string(),
            Some("Standard user access".to_string()),
            Utc::now(),
            Utc::now(),
        )
    }

    fn create_test_admin_role() -> Role {
        Role::new(
            admin_role_id(),
            "Admin".to_string(),
            Some("Full system access".to_string()),
            Utc::now(),
            Utc::now(),
        )
    }

    #[test]
    fn test_create_new_user() {
        let user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            create_test_user_role(),
        );

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.is_active);
        assert!(!user.is_admin());
        assert!(user.role().is_user());
    }

    #[test]
    fn test_admin_user() {
        let user = User::new(
            "admin".to_string(),
            "admin@example.com".to_string(),
            "hashed_password".to_string(),
            create_test_admin_role(),
        );

        assert!(user.is_admin());
        assert!(user.role().is_admin());
    }

    #[test]
    fn test_update_username() {
        let mut user = User::new(
            "oldname".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
            create_test_user_role(),
        );

        let old_updated_at = user.updated_at;

        user.update_username("newname".to_string());

        assert_eq!(user.username, "newname");
        assert!(user.updated_at > old_updated_at);
    }

    #[test]
    fn test_deactivate_user() {
        let mut user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
            create_test_user_role(),
        );

        assert!(user.is_active());

        user.deactivate();

        assert!(!user.is_active());
    }

    #[test]
    fn test_has_role() {
        let user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
            create_test_user_role(),
        );

        assert!(user.has_role(user_role_id()));
        assert!(!user.has_role(admin_role_id()));
    }
}
