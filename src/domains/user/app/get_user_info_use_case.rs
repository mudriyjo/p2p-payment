use std::sync::Arc;

use uuid::Uuid;

use crate::{common::error::AppError, domains::user::{UserRepository, domain::model::User}};

pub struct GetUserInfoUseCase {
    user_repository: Arc<dyn UserRepository>
}

impl GetUserInfoUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        requester_id: Option<Uuid>, // Кто запрашивает
    ) -> Result<User, AppError> {
        tracing::debug!("Fetching user {} by requester {:?}", user_id, requester_id);

        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound(format!("User {} not found", user_id)))?;

        // TODO: change to real check
        if let Some(req_id) = requester_id {
            self.check_access_permission(req_id, &user).await?;
        }


        Ok(user)
    }

    async fn check_access_permission(
        &self,
        requester_id: Uuid,
        target_user: &User,
    ) -> Result<(), AppError> {
        // TODO: implement logic
        Ok(())
    }

    /// Получить список пользователей с фильтрацией
    pub async fn list(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, AppError> {
        tracing::debug!("Listing users with limit={}, offset={}", limit, offset);

        if limit > 100 {
            return Err(AppError::ValidationError(
                "Limit cannot exceed 100".to_string(),
            ));
        }

        self.user_repository.list(limit, offset).await
    }
}