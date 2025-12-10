use super::model::Role;
use crate::common::error::AppError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    /// Find a role by its ID
    async fn find_by_id(&self, role_id: Uuid) -> Result<Option<Role>, AppError>;

    /// Find a role by its name
    async fn find_by_name(&self, role_name: &str) -> Result<Option<Role>, AppError>;

    /// List all roles
    async fn list_all(&self) -> Result<Vec<Role>, AppError>;
}
