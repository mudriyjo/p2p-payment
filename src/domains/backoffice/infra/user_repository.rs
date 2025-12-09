use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, QuerySelect, Set, ActiveModelTrait, PaginatorTrait};
use uuid::Uuid;
use chrono::Utc;
use crate::common::error::AppError;
use crate::domains::backoffice::domain::{
    model::User,
    repository::UserRepository,
};
use crate::domains::backoffice::role::model::Role;
use super::user_entity::{self, Entity as UserEntity};
use super::super::role::entity::{self as role_entity, Entity as RoleEntity};

pub struct PostgresUserRepository {
    db: DatabaseConnection,
}

impl PostgresUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn entity_to_domain(user_entity: user_entity::Model, role_entity: role_entity::Model) -> User {
        let role = Role {
            role_id: role_entity.role_id,
            role_name: role_entity.role_name,
            role_description: role_entity.role_description,
            created_at: role_entity.created_at.with_timezone(&Utc),
            updated_at: role_entity.updated_at.with_timezone(&Utc),
        };

        User {
            id: user_entity.id,
            username: user_entity.username,
            email: user_entity.email,
            password_hash: user_entity.password_hash,
            is_active: user_entity.is_active,
            role,
            created_at: user_entity.created_at.with_timezone(&Utc),
            updated_at: user_entity.updated_at.with_timezone(&Utc),
        }
    }

    fn domain_to_active_model(user: User) -> user_entity::ActiveModel {
        user_entity::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            is_active: Set(user.is_active),
            role_id: Set(user.role.role_id),
            created_at: Set(user.created_at.into()),
            updated_at: Set(user.updated_at.into()),
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let result = UserEntity::find_by_id(id)
            .find_also_related(RoleEntity)
            .one(&self.db)
            .await?;

        match result {
            Some((user, Some(role))) => Ok(Some(Self::entity_to_domain(user, role))),
            _ => Ok(None),
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let result = UserEntity::find()
            .filter(user_entity::Column::Username.eq(username))
            .find_also_related(RoleEntity)
            .one(&self.db)
            .await?;

        match result {
            Some((user, Some(role))) => Ok(Some(Self::entity_to_domain(user, role))),
            _ => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let result = UserEntity::find()
            .filter(user_entity::Column::Email.eq(email))
            .find_also_related(RoleEntity)
            .one(&self.db)
            .await?;

        match result {
            Some((user, Some(role))) => Ok(Some(Self::entity_to_domain(user, role))),
            _ => Ok(None),
        }
    }



    async fn exists_by_username(&self, username: &str) -> Result<bool, AppError> {
        let count = UserEntity::find()
            .filter(user_entity::Column::Username.eq(username))
            .count(&self.db)
            .await?;

        Ok(count > 0)
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        let count = UserEntity::find()
            .filter(user_entity::Column::Email.eq(email))
            .count(&self.db)
            .await?;

        Ok(count > 0)
    }

    async fn create(&self, user: User) -> Result<User, AppError> {
        let user_id = user.id;
        let active_model = Self::domain_to_active_model(user);
        active_model.insert(&self.db).await?;

        // Re-fetch with role
        self.find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found after creation", user_id)))
    }

    async fn update(&self, user: User) -> Result<User, AppError> {
        let user_id = user.id;
        let active_model = Self::domain_to_active_model(user);
        active_model.update(&self.db).await?;

        // Re-fetch with role
        self.find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found after update", user_id)))
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = UserEntity::delete_by_id(id).exec(&self.db).await?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound(format!("User {} not found", id)));
        }

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        let results = UserEntity::find()
            .find_also_related(RoleEntity)
            .order_by_desc(user_entity::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(&self.db)
            .await?;

        Ok(results
            .into_iter()
            .filter_map(|(user, role)| role.map(|r| Self::entity_to_domain(user, r)))
            .collect())
    }

    async fn search(&self, query: &str, limit: i64) -> Result<Vec<User>, AppError> {
        let search_pattern = format!("%{}%", query);

        let results = UserEntity::find()
            .find_also_related(RoleEntity)
            .filter(
                user_entity::Column::Username.like(&search_pattern)
                    .or(user_entity::Column::Email.like(&search_pattern))
            )
            .order_by_asc(user_entity::Column::Username)
            .limit(limit as u64)
            .all(&self.db)
            .await?;

        Ok(results
            .into_iter()
            .filter_map(|(user, role)| role.map(|r| Self::entity_to_domain(user, r)))
            .collect())
    }

    async fn is_admin(&self, id: Uuid) -> Result<bool, AppError> {
        let user = self.find_by_id(id).await?;
        Ok(user.map(|u| u.is_admin()).unwrap_or(false))
    }

    async fn count(&self) -> Result<i64, AppError> {
        let count = UserEntity::find().count(&self.db).await?;
        Ok(count as i64)
    }
}

#[cfg(test)]
mod tests {
    // Integration тесты для repository обычно требуют настройки тестовой БД
    // См. tests/test_user_repository.rs
}