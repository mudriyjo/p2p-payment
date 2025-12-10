use super::entity::{self, Entity as RoleEntity};
use super::model::Role;
use super::repository::RoleRepository;
use crate::common::error::AppError;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

pub struct PostgresRoleRepository {
    db: DatabaseConnection,
}

impl PostgresRoleRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn entity_to_domain(entity: entity::Model) -> Role {
        Role {
            role_id: entity.role_id,
            role_name: entity.role_name,
            role_description: entity.role_description,
            created_at: entity.created_at.with_timezone(&Utc),
            updated_at: entity.updated_at.with_timezone(&Utc),
        }
    }
}

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    async fn find_by_id(&self, role_id: Uuid) -> Result<Option<Role>, AppError> {
        let role = RoleEntity::find_by_id(role_id).one(&self.db).await?;

        Ok(role.map(Self::entity_to_domain))
    }

    async fn find_by_name(&self, role_name: &str) -> Result<Option<Role>, AppError> {
        let role = RoleEntity::find()
            .filter(entity::Column::RoleName.eq(role_name))
            .one(&self.db)
            .await?;

        Ok(role.map(Self::entity_to_domain))
    }

    async fn list_all(&self) -> Result<Vec<Role>, AppError> {
        let roles = RoleEntity::find()
            .order_by_asc(entity::Column::RoleName)
            .all(&self.db)
            .await?;

        Ok(roles.into_iter().map(Self::entity_to_domain).collect())
    }
}

#[cfg(test)]
mod tests {
    // Integration tests for repository would require database setup
}
