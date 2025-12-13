use std::sync::Arc;

use crate::{
    common::{error::AppError, hash_utils::hash_password},
    domains::backoffice::{
        domain::user::User, dto::user_dto::CreateUserRequest, role::repository::RoleRepository,
        UserRepository,
    },
};

pub struct CreateUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    role_repository: Arc<dyn RoleRepository>,
}

impl CreateUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        role_repository: Arc<dyn RoleRepository>,
    ) -> Self {
        Self {
            user_repository,
            role_repository,
        }
    }

    pub async fn execute(&self, request: CreateUserRequest) -> Result<User, AppError> {
        tracing::debug!(
            "Creating user '{}' with role_id {}",
            request.username,
            request.role_id
        );

        // Validate role exists
        let role = self
            .role_repository
            .find_by_id(request.role_id)
            .await?
            .ok_or(AppError::NotFound(format!(
                "Role {} not found",
                request.role_id
            )))?;

        // Check username uniqueness
        if self
            .user_repository
            .exists_by_username(&request.username)
            .await?
        {
            return Err(AppError::ValidationError(format!(
                "Username '{}' already exists",
                request.username
            )));
        }

        // Check email uniqueness
        if self.user_repository.exists_by_email(&request.email).await? {
            return Err(AppError::ValidationError(format!(
                "Email '{}' already exists",
                request.email
            )));
        }

        // Hash password
        let password_hash = hash_password(&request.password)?;

        // Create user domain model
        let user = User::new(request.username, request.email, password_hash, role);

        // Save to database
        let created_user = self.user_repository.create(user).await?;

        tracing::info!("User {} created successfully", created_user.id);

        Ok(created_user)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would require database setup
}
