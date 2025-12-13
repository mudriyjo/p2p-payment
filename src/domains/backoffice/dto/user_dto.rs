use crate::domains::backoffice::domain::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoleInfo {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub role_id: String,
    
    #[schema(example = "Admin")]
    pub role_name: String,
    
    #[schema(example = "Administrator role")]
    pub role_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    
    #[schema(example = "testuser")]
    pub username: String,
    
    #[schema(example = "test@example.com")]
    pub email: String,

    #[schema(example = "true")]
    pub is_active: bool,

    pub role: RoleInfo,

    #[serde(with = "crate::common::time_formater")]
    #[schema(example = "2024-01-01T12:00:00Z")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "crate::common::time_formater")]
    #[schema(example = "2024-01-01T12:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            is_active: user.is_active,
            role: RoleInfo {
                role_id: user.role.role_id.to_string(),
                role_name: user.role.role_name,
                role_description: user.role.role_description,
            },
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
    pub role_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role_id: Option<Uuid>,
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
    use crate::domains::backoffice::role::model::{user_role_id, Role};
    use chrono::Utc;

    #[test]
    fn test_user_response_from_user() {
        let role = Role::new(
            user_role_id(),
            "User".to_string(),
            Some("Standard user access".to_string()),
            Utc::now(),
            Utc::now(),
        );

        let user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hash".to_string(),
            role,
        );

        let response = UserResponse::from(user.clone());

        assert_eq!(response.id, user.id.to_string());
        assert_eq!(response.username, "testuser");
        assert_eq!(response.email, "test@example.com");
        assert!(response.is_active);
        assert_eq!(response.role.role_name, "User");
        assert!(!user.is_admin());
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
