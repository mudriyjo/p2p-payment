use std::sync::Arc;

use uuid::Uuid;

use crate::{
    common::error::AppError,
    domains::backoffice::{domain::model::User, UserRepository},
};

pub struct GetUserInfoUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetUserInfoUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<User, AppError> {
        tracing::debug!("Fetching user {}", user_id);

        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound(format!("User {} not found", user_id)))?;

        Ok(user)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        tracing::debug!("Listing users with limit={}, offset={}", limit, offset);

        if limit > 100 {
            return Err(AppError::ValidationError(
                "Limit cannot exceed 100".to_string(),
            ));
        }

        self.user_repository.list(limit, offset).await
    }

    pub async fn search(&self, query: &str, limit: Option<i64>) -> Result<Vec<User>, AppError> {
        let search_limit = limit.unwrap_or(20);

        if search_limit > 100 {
            return Err(AppError::ValidationError(
                "Limit cannot exceed 100".to_string(),
            ));
        }

        tracing::debug!(
            "Searching users with query='{}', limit={}",
            query,
            search_limit
        );

        self.user_repository.search(query, search_limit).await
    }
}
