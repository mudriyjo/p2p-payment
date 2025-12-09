use std::sync::Arc;
use sea_orm::DatabaseConnection;

// Repositories
use crate::domains::user::domain::repository::UserRepository;
use crate::domains::user::infra::user_repository::PostgresUserRepository;

// User Use Cases
use crate::domains::user::app::get_user_info_use_case::GetUserInfoUseCase;

// Services
use crate::common::jwt::JwtService;
use crate::common::Config;

pub struct AppState {
    pub config: Config,
    pub user_repository: Arc<dyn UserRepository>,
    pub jwt_service: Arc<JwtService>,
    pub user_get_use_case: Arc<GetUserInfoUseCase>,
    // pub user_update_profile_use_case: Arc<UpdateProfileUseCase>,
    // pub user_delete_use_case: Arc<DeleteUserUseCase>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config: Config) -> Self {
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(PostgresUserRepository::new(db));

        let jwt_service = Arc::new(JwtService::new(&config.jwt_secret_key));

        let user_get_use_case = Arc::new(GetUserInfoUseCase::new(
            Arc::clone(&user_repository),
        ));

        // let user_update_profile_use_case = Arc::new(UpdateProfileUseCase::new(
        //     Arc::clone(&user_repository),
        // ));

        // let user_delete_use_case = Arc::new(DeleteUserUseCase::new(
        //     Arc::clone(&user_repository)
        // ));

        Self {
            config,
            user_repository,
            jwt_service,
            user_get_use_case,
            // user_update_profile_use_case,
            // user_delete_use_case,
        }
    }
}