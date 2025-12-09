mod api {
    pub mod handler;
    pub mod router;
}

pub mod app {
    pub mod get_user_info_use_case;
}

pub mod domain {
    pub mod model;
    pub mod repository;
}

pub mod dto {
    pub mod user_dto;
}

mod infra {
    pub mod user_repository;
}

pub use api::router::{public_user_routes, protected_user_routes};//, UserApiDoc};
pub use domain::repository::UserRepository;
pub use infra::user_repository::PostgresUserRepository;