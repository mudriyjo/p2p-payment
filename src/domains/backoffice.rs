mod api {
    pub mod handler;
    pub mod router;
}

pub mod app {
    pub mod create_user_use_case;
    pub mod delete_user_use_case;
    pub mod get_user_info_use_case;
    pub mod update_user_use_case;
}

pub mod domain {
    pub mod model;
    pub mod repository;
}

pub mod dto {
    pub mod user_dto;
}

pub mod infra {
    pub mod user_entity;
    pub mod user_repository;
}

pub mod role;

pub use api::router::{protected_user_routes, UserApiDoc};
pub use domain::repository::UserRepository;
pub use infra::user_repository::PostgresUserRepository;
pub use role::{PostgresRoleRepository, RoleRepository};
