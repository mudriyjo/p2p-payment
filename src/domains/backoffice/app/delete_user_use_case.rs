use std::sync::Arc;

use uuid::Uuid;

use crate::{common::error::AppError, domains::backoffice::UserRepository};

pub struct DeleteUserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl DeleteUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<(), AppError> {
        tracing::debug!("Deleting user {}", user_id);

        // Verify user exists before attempting deletion
        let _user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound(format!("User {} not found", user_id)))?;

        // Delete from database
        self.user_repository.delete(user_id).await?;

        tracing::info!("User {} deleted successfully", user_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would require database setup
}
