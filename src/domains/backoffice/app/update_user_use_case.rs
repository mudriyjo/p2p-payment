use std::sync::Arc;

use uuid::Uuid;

use crate::{
    common::error::AppError,
    domains::backoffice::{
        domain::user::User, dto::user_dto::UpdateUserRequest, role::repository::RoleRepository,
        UserRepository,
    },
};

pub struct UpdateUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    role_repository: Arc<dyn RoleRepository>,
}

impl UpdateUserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        role_repository: Arc<dyn RoleRepository>,
    ) -> Self {
        Self {
            user_repository,
            role_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        request: UpdateUserRequest,
    ) -> Result<User, AppError> {
        tracing::debug!("Updating user {}", user_id);

        // Fetch existing user
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound(format!("User {} not found", user_id)))?;

        // Update username if provided and different
        if let Some(new_username) = request.username {
            if new_username != user.username {
                // Check uniqueness
                if self
                    .user_repository
                    .exists_by_username(&new_username)
                    .await?
                {
                    return Err(AppError::ValidationError(format!(
                        "Username '{}' already exists",
                        new_username
                    )));
                }
                user.update_username(new_username);
            }
        }

        // Update email if provided and different
        if let Some(new_email) = request.email {
            if new_email != user.email {
                // Check uniqueness
                if self.user_repository.exists_by_email(&new_email).await? {
                    return Err(AppError::ValidationError(format!(
                        "Email '{}' already exists",
                        new_email
                    )));
                }
                user.update_email(new_email);
            }
        }

        // Update role if provided and different
        if let Some(new_role_id) = request.role_id {
            if new_role_id != user.role.role_id {
                // Validate role exists
                let role = self.role_repository.find_by_id(new_role_id).await?.ok_or(
                    AppError::NotFound(format!("Role {} not found", new_role_id)),
                )?;

                user.update_role(role);
            }
        }

        // Save updated user to database
        let updated_user = self.user_repository.update(user).await?;

        tracing::info!("User {} updated successfully", updated_user.id);

        Ok(updated_user)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would require database setup
}
