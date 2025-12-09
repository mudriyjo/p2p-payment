use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, QuerySelect, Set, ActiveModelTrait, PaginatorTrait};
use uuid::Uuid;
use chrono::Utc;
use crate::common::error::AppError;
use crate::domains::user::domain::{
    model::User,
    repository::UserRepository,
};
use super::user_entity::{self, Entity as UserEntity};

pub struct PostgresUserRepository {
    db: DatabaseConnection,
}

impl PostgresUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn entity_to_domain(entity: user_entity::Model) -> User {
        User {
            id: entity.id,
            username: entity.username,
            email: entity.email,
            password_hash: entity.password_hash,
            is_active: entity.is_active,
            is_admin: entity.is_admin,
            created_at: entity.created_at.with_timezone(&Utc),
            updated_at: entity.updated_at.with_timezone(&Utc),
        }
    }

    fn domain_to_active_model(user: User) -> user_entity::ActiveModel {
        user_entity::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            is_active: Set(user.is_active),
            is_admin: Set(user.is_admin),
            created_at: Set(user.created_at.into()),
            updated_at: Set(user.updated_at.into()),
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = UserEntity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(user.map(Self::entity_to_domain))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = UserEntity::find()
            .filter(user_entity::Column::Username.eq(username))
            .one(&self.db)
            .await?;

        Ok(user.map(Self::entity_to_domain))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = UserEntity::find()
            .filter(user_entity::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        Ok(user.map(Self::entity_to_domain))
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
        let active_model = Self::domain_to_active_model(user);
        let result = active_model.insert(&self.db).await?;
        Ok(Self::entity_to_domain(result))
    }

    async fn update(&self, user: User) -> Result<User, AppError> {
        let active_model = Self::domain_to_active_model(user);
        let result = active_model.update(&self.db).await?;
        Ok(Self::entity_to_domain(result))
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = UserEntity::delete_by_id(id).exec(&self.db).await?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound(format!("User {} not found", id)));
        }

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        let users = UserEntity::find()
            .order_by_desc(user_entity::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(&self.db)
            .await?;

        Ok(users.into_iter().map(Self::entity_to_domain).collect())
    }

    async fn search(&self, query: &str, limit: i64) -> Result<Vec<User>, AppError> {
        use sea_orm::sea_query::{SimpleExpr};

        let search_pattern = format!("%{}%", query);

        let users = UserEntity::find()
            .filter(
                user_entity::Column::Username.like(&search_pattern)
                    .or(user_entity::Column::Email.like(&search_pattern))
            )
            .order_by_asc(user_entity::Column::Username)
            .limit(limit as u64)
            .all(&self.db)
            .await?;

        Ok(users.into_iter().map(Self::entity_to_domain).collect())
    }

    async fn is_admin(&self, id: Uuid) -> Result<bool, AppError> {
        let user = UserEntity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(user.map(|u| u.is_admin).unwrap_or(false))
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