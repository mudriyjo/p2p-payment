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
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password_hash: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            is_active: true,
            is_admin: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
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
    use super::*;

    #[test]
    fn test_create_new_user() {
        let user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hashed_password".to_string(),
        );

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.is_active);
        assert!(!user.is_admin);
    }

    #[test]
    fn test_update_username() {
        let mut user = User::new(
            "oldname".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
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
        );

        assert!(user.is_active());
        
        user.deactivate();
        
        assert!(!user.is_active());
    }
}