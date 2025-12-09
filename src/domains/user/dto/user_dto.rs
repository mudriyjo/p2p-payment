use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domains::user::domain::model::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_admin: bool,
    #[serde(with = "crate::common::time_formater")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::common::time_formater")]
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            is_active: user.is_active,
            is_admin: user.is_admin,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    pub search: Option<String>,
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserRequest {
    pub password_confirmation: Option<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_response_from_user() {
        let user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
        );

        let response = UserResponse::from(user.clone());

        assert_eq!(response.id, user.id.to_string());
        assert_eq!(response.username, "testuser");
        assert_eq!(response.email, "test@example.com");
        assert!(response.is_active);
        assert!(!response.is_admin);
    }

    #[test]
    fn test_list_users_query_defaults() {
        let json = r#"{}"#;
        let query: ListUsersQuery = serde_json::from_str(json).unwrap();

        assert_eq!(query.limit, 20);
        assert_eq!(query.offset, 0);
        assert!(query.search.is_none());
    }
}