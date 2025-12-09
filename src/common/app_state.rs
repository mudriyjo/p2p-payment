use std::sync::Arc;
use sqlx::PgPool;

// Repositories
use crate::domains::user::domain::repository::UserRepository;
use crate::domains::user::PostgresUserRepository;

// User Use Cases
use crate::domains::user::app::{
    GetUserInfoUseCase,
    // UpdateProfileUseCase,
    // DeleteUserUseCase,
};

// Services
use crate::common::jwt::JwtService;

pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub jwt_service: Arc<JwtService>,
    pub user_get_use_case: Arc<GetUserInfoUseCase>,
    // pub user_update_profile_use_case: Arc<UpdateProfileUseCase>,
    // pub user_delete_use_case: Arc<DeleteUserUseCase>,
}

impl AppState {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        let user_repository: Arc<dyn UserRepository> = 
            Arc::new(PostgresUserRepository::new(pool.clone()));
        
        let jwt_service = Arc::new(JwtService::new(&jwt_secret));

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
            user_repository,
            jwt_service,
            user_get_use_case,
            // user_update_profile_use_case,
            // user_delete_use_case,
        }
    }
}