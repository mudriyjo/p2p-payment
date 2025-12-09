use async_trait::async_trait;
use uuid::Uuid;
use crate::common::error::AppError;
use super::model::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

    async fn exists_by_username(&self, username: &str) -> Result<bool, AppError>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, AppError>;

    async fn create(&self, user: User) -> Result<User, AppError>;
    async fn update(&self, user: User) -> Result<User, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError>;
    async fn search(&self, query: &str, limit: i64) -> Result<Vec<User>, AppError>;

    async fn is_admin(&self, id: Uuid) -> Result<bool, AppError>;
    async fn count(&self) -> Result<i64, AppError>;
}

// Mock для тестирования (используется в use cases)
#[cfg(test)]
use mockall::mock;

#[cfg(test)]
mock! {
    pub UserRepository {}

    #[async_trait]
    impl UserRepository for UserRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
        async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
        async fn exists_by_username(&self, username: &str) -> Result<bool, AppError>;
        async fn exists_by_email(&self, email: &str) -> Result<bool, AppError>;
        async fn create(&self, user: User) -> Result<User, AppError>;
        async fn update(&self, user: User) -> Result<User, AppError>;
        async fn delete(&self, id: Uuid) -> Result<(), AppError>;
        async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError>;
        async fn search(&self, query: &str, limit: i64) -> Result<Vec<User>, AppError>;
        async fn is_admin(&self, id: Uuid) -> Result<bool, AppError>;
        async fn count(&self) -> Result<i64, AppError>;
    }
}