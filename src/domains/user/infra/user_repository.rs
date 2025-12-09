use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::common::error::AppError;
use crate::domains::user::domain::{
    model::User,
    repository::UserRepository,
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, bio, phone, 
                   avatar_file_id, is_active, is_admin, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, bio, phone,
                   avatar_file_id, is_active, is_admin, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, bio, phone,
                   avatar_file_id, is_active, is_admin, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }



    async fn exists_by_username(&self, username: &str) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE username = $1) as "exists!"
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists)
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as "exists!"
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists)
    }

    async fn create(&self, user: User) -> Result<User, AppError> {
        let created_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                id, username, email, password_hash, bio, phone,
                avatar_file_id, is_active, is_admin, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, username, email, password_hash, bio, phone,
                      avatar_file_id, is_active, is_admin, created_at, updated_at
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.bio,
            user.phone,
            user.avatar_file_id,
            user.is_active,
            user.is_admin,
            user.created_at,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_user)
    }

    async fn update(&self, user: User) -> Result<User, AppError> {
        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = $2,
                email = $3,
                password_hash = $4,
                bio = $5,
                phone = $6,
                avatar_file_id = $7,
                is_active = $8,
                is_admin = $9,
                updated_at = $10
            WHERE id = $1
            RETURNING id, username, email, password_hash, bio, phone,
                      avatar_file_id, is_active, is_admin, created_at, updated_at
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.bio,
            user.phone,
            user.avatar_file_id,
            user.is_active,
            user.is_admin,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("User {} not found", id)));
        }

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, bio, phone,
                   avatar_file_id, is_active, is_admin, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn search(&self, query: &str, limit: i64) -> Result<Vec<User>, AppError> {
        let search_pattern = format!("%{}%", query);
        
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, bio, phone,
                   avatar_file_id, is_active, is_admin, created_at, updated_at
            FROM users
            WHERE username ILIKE $1 OR email ILIKE $1
            ORDER BY username
            LIMIT $2
            "#,
            search_pattern,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn is_admin(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT is_admin FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.is_admin).unwrap_or(false))
    }

    async fn count(&self) -> Result<i64, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!" FROM users
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count)
    }
}

#[cfg(test)]
mod tests {
    // Integration тесты для repository обычно требуют настройки тестовой БД
    // См. tests/test_user_repository.rs
}